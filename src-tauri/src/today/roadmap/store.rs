use super::{model::*, *};
use chrono::{Datelike, Duration, TimeZone};
use sqlx::{SqliteConnection, SqlitePool};
pub(super) fn week(value: &str) -> Result<NaiveDate> {
    let day = date(value)?;
    if day.weekday() != chrono::Weekday::Mon {
        return Err(DbError::invalid());
    }
    day.checked_add_signed(Duration::days(7))
        .ok_or_else(DbError::invalid)?;
    Ok(day)
}
pub(super) fn table(kind: &str) -> Result<&'static str> {
    match kind {
        "journey" => Ok("journeys"),
        "phase" => Ok("journey_phases"),
        "subject" => Ok("subjects"),
        "quest" => Ok("quests"),
        _ => Err(DbError::invalid()),
    }
}
pub(super) fn decode<T: serde::de::DeserializeOwned>(value: &str) -> Result<T> {
    serde_json::from_str(value).map_err(|_| {
        DbError::new(
            "DATABASE_INVALID_RECORD",
            "规划记录格式无效，请保留数据并检查。",
        )
    })
}
pub(super) async fn snapshot(pool: &SqlitePool, week_start: &str) -> Result<Snapshot> {
    let mut connection = pool.acquire().await?;
    load(&mut connection, week_start).await
}
pub(super) async fn load(connection: &mut SqliteConnection, week_start: &str) -> Result<Snapshot> {
    let first = week(week_start)?;
    let until = (first + Duration::days(7)).to_string();
    let mut entities = Vec::new();
    for kind in ["journey", "phase", "subject", "quest"] {
        let parent = if kind == "journey" {
            "NULL AS journey_id"
        } else {
            "journey_id"
        };
        let rows=sqlx::query(&format!("SELECT id,title,{parent},planning_data,revision,deleted_at,archived_at FROM {} ORDER BY created_at,id LIMIT 1000",table(kind)?)).fetch_all(&mut *connection).await?;
        for row in rows {
            entities.push(Entity {
                id: row.get("id"),
                kind: kind.into(),
                title: row.get("title"),
                journey_id: row.get("journey_id"),
                data: decode(&row.get::<String, _>("planning_data"))?,
                revision: row.get("revision"),
                deleted_at: row.get("deleted_at"),
                archived_at: row.get("archived_at"),
            })
        }
    }
    for entity in &mut entities {
        if entity.data.progress_mode == "tasks" {
            let column = match entity.kind.as_str() {
                "journey" => "journey_id",
                "subject" => "subject_id",
                "quest" => "quest_id",
                _ => continue,
            };
            let (total,done):(i64,i64)=sqlx::query_as(&format!("SELECT COUNT(*),COALESCE(SUM(CASE WHEN lifecycle='done' THEN 1 ELSE 0 END),0) FROM tasks WHERE {column}=? AND deleted_at IS NULL AND lifecycle NOT IN ('cancelled','skipped','moved')")).bind(&entity.id).fetch_one(&mut *connection).await?;
            entity.data.progress = if total == 0 { 0 } else { done * 100 / total };
        }
    }
    let rows=sqlx::query("SELECT * FROM tasks WHERE lifecycle IN ('planned','active') AND deleted_at IS NULL ORDER BY due_date,priority DESC,id LIMIT 1000").fetch_all(&mut *connection).await?;
    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(PlanningTask {
            id: row.get("id"),
            title: row.get("title"),
            remaining_minutes: row.get("remaining_minutes"),
            priority: row.get("priority"),
            deadline: row.get("due_date"),
            journey_id: row.get("journey_id"),
            subject_id: row.get("subject_id"),
            quest_id: row.get("quest_id"),
            revision: row.get("revision"),
            options: decode(&row.get::<String, _>("planning_data"))?,
        })
    }
    let preferences = if let Some(row) =
        sqlx::query("SELECT * FROM planning_preferences WHERE id=1")
            .fetch_optional(&mut *connection)
            .await?
    {
        Preferences {
            weekday_minutes: row.get("weekday_minutes"),
            weekend_minutes: row.get("weekend_minutes"),
            side_quest_max: row.get("side_quest_max"),
            planning_target: row.get("planning_target"),
            revision: row.get("revision"),
        }
    } else {
        let mut p = Preferences::default();
        if let Some(value)=sqlx::query_scalar::<_,String>("SELECT value_text FROM settings WHERE key='daily_capacity_minutes' AND deleted_at IS NULL").fetch_optional(&mut *connection).await?{p.weekday_minutes=value.parse().unwrap_or(210);p.weekend_minutes=p.weekday_minutes}
        p
    };
    let days = sqlx::query("SELECT * FROM capacity_days WHERE date>=? AND date<? ORDER BY date")
        .bind(week_start)
        .bind(&until)
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|r| CapacityDay {
            date: r.get("date"),
            minutes: r.get("minutes"),
            status: r.get("status"),
        })
        .collect();
    let blocks=sqlx::query("SELECT task_id,date,minutes FROM planning_blocks WHERE date>=? AND date<? ORDER BY date,task_id").bind(week_start).bind(&until).fetch_all(&mut *connection).await?.into_iter().map(|r|Allocation{task_id:r.get("task_id"),date:r.get("date"),minutes:r.get("minutes")}).collect();
    let mut decisions = Vec::new();
    for row in sqlx::query("SELECT * FROM planning_decisions WHERE week_start=? ORDER BY created_at DESC,id DESC LIMIT 50").bind(week_start).fetch_all(&mut *connection).await?{decisions.push(Decision{id:row.get("id"),week_start:row.get("week_start"),status:row.get("status"),reason:row.get("reason"),before:decode(&row.get::<String,_>("before_json"))?,after:decode(&row.get::<String,_>("after_json"))?,created_at:row.get("created_at"),updated_at:row.get("updated_at")})}
    let mut rules = Vec::new();
    for r in sqlx::query("SELECT * FROM exam_rule_snapshots WHERE year IS NOT NULL ORDER BY year DESC,captured_at DESC LIMIT 100").fetch_all(&mut *connection).await?{rules.push(Rule{id:r.get("id"),input:RuleInput{journey_id:r.get("journey_id"),school:r.get("school"),college:r.get("college"),major:r.get("major"),year:r.get("year"),written_exam:r.get("written_exam"),interview:r.get("interview"),source_url:r.get("source_url"),body:r.get("body"),notes:r.get("notes")},captured_at:r.get("captured_at"),content_hash:r.get("content_hash"),deleted_at:r.get("deleted_at")})}
    let mut snapshot = Snapshot {
        week_start: week_start.into(),
        entities,
        tasks,
        preferences,
        days,
        blocks,
        decisions,
        rules,
        budgets: Vec::new(),
    };
    snapshot.budgets = budgets(connection, &snapshot).await?;
    Ok(snapshot)
}
fn live_entity(entity: &Entity, all: &[Entity]) -> bool {
    let mut current = Some(entity);
    let mut seen = std::collections::BTreeSet::new();
    while let Some(e) = current {
        if e.deleted_at.is_some()
            || e.archived_at.is_some()
            || e.data.status != "active"
            || !seen.insert(&e.id)
        {
            return false;
        }
        if let Some(journey) = &e.journey_id {
            if !all.iter().any(|j| {
                &j.id == journey
                    && j.deleted_at.is_none()
                    && j.archived_at.is_none()
                    && j.data.status == "active"
            }) {
                return false;
            }
        }
        current = match &e.data.parent_id {
            Some(id) => match all.iter().find(|p| &p.id == id) {
                Some(parent) => Some(parent),
                None => return false,
            },
            None => None,
        };
    }
    true
}
pub(super) fn live_entities(snapshot: &Snapshot) -> Vec<Entity> {
    snapshot
        .entities
        .iter()
        .filter(|e| live_entity(e, &snapshot.entities))
        .cloned()
        .collect()
}
pub(super) fn live_tasks(snapshot: &Snapshot) -> Vec<PlanningTask> {
    snapshot
        .tasks
        .iter()
        .filter(|t| {
            [&t.journey_id, &t.subject_id, &t.quest_id]
                .into_iter()
                .flatten()
                .all(|id| {
                    snapshot
                        .entities
                        .iter()
                        .any(|e| &e.id == id && live_entity(e, &snapshot.entities))
                })
        })
        .cloned()
        .collect()
}
pub(super) async fn budgets(
    connection: &mut SqliteConnection,
    snapshot: &Snapshot,
) -> Result<Vec<DayBudget>> {
    let first = week(&snapshot.week_start)?;
    let mut days = Vec::new();
    let events = sqlx::query(
        "SELECT event_date,starts_at,ends_at FROM calendar_events WHERE deleted_at IS NULL",
    )
    .fetch_all(&mut *connection)
    .await?;
    for offset in 0..7 {
        let day = first + Duration::days(offset);
        let key = day.to_string();
        let override_day = snapshot.days.iter().find(|d| d.date == key);
        let capacity =
            override_day
                .map(|d| d.minutes)
                .unwrap_or(if day.weekday().number_from_monday() <= 5 {
                    snapshot.preferences.weekday_minutes
                } else {
                    snapshot.preferences.weekend_minutes
                });
        let status = override_day
            .map(|d| d.status.clone())
            .unwrap_or_else(|| "normal".into());
        let boundary = |d: NaiveDate| -> Result<DateTime<Utc>> {
            Local
                .from_local_datetime(&d.and_hms_opt(0, 0, 0).ok_or_else(DbError::invalid)?)
                .earliest()
                .map(|x| x.with_timezone(&Utc))
                .ok_or_else(DbError::invalid)
        };
        let start = boundary(day)?;
        let end = boundary(day + Duration::days(1))?;
        let mut intervals = Vec::new();
        let mut all_day = false;
        for e in &events {
            if e.get::<Option<String>, _>("event_date").as_deref() == Some(&key) {
                all_day = true
            }
            if let (Some(s), Some(f)) = (
                e.get::<Option<String>, _>("starts_at"),
                e.get::<Option<String>, _>("ends_at"),
            ) {
                let s = DateTime::parse_from_rfc3339(&s)
                    .map_err(|_| DbError::invalid())?
                    .with_timezone(&Utc)
                    .max(start);
                let f = DateTime::parse_from_rfc3339(&f)
                    .map_err(|_| DbError::invalid())?
                    .with_timezone(&Utc)
                    .min(end);
                if f > s {
                    intervals.push((s, f))
                }
            }
        }
        intervals.sort();
        let mut merged: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();
        for (s, f) in intervals {
            if let Some(last) = merged.last_mut() {
                if s <= last.1 {
                    last.1 = last.1.max(f);
                    continue;
                }
            }
            merged.push((s, f))
        }
        let fixed = if all_day {
            capacity
        } else {
            ((merged
                .iter()
                .map(|(s, f)| (*f - *s).num_seconds())
                .sum::<i64>()
                + 59)
                / 60)
                .min(capacity)
        };
        days.push(DayBudget {
            date: key,
            capacity,
            fixed_minutes: fixed,
            available: if day < Local::now().date_naive() {
                0
            } else {
                (capacity - fixed).max(0)
            },
            status,
        });
    }
    Ok(days)
}
