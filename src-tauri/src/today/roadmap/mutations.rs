use super::model::conflict;
use super::{
    model::*,
    store::{decode, table},
    *,
};
pub(super) async fn reference(
    tx: &mut Transaction<'_, Sqlite>,
    kind: &str,
    id: &str,
) -> Result<()> {
    validate_id(id)?;
    let n: i64 = sqlx::query_scalar(&format!(
        "SELECT count(*) FROM {} WHERE id=? AND deleted_at IS NULL AND archived_at IS NULL",
        table(kind)?
    ))
    .bind(id)
    .fetch_one(&mut **tx)
    .await?;
    if n != 1 {
        return Err(conflict());
    }
    Ok(())
}
pub(super) fn validate_entity(kind: &str, input: &EntityInput) -> Result<()> {
    table(kind)?;
    let d = &input.data;
    if input.title.trim().is_empty()
        || input.title.len() > 1000
        || d.description.len() > 100000
        || d.category.len() > 100
        || d.preparation_phase.len() > 1000
        || !(0..=3).contains(&d.priority)
        || !(0..=100).contains(&d.progress)
        || !(0..=10080).contains(&d.maximum_minutes)
        || d.minimum_minutes < 0
        || d.minimum_minutes > d.preferred_minutes
        || d.preferred_minutes > d.maximum_minutes
        || !(0..=10000).contains(&d.position)
        || !["active", "paused", "done"].contains(&d.status.as_str())
        || !["manual", "tasks"].contains(&d.progress_mode.as_str())
        || !["sage", "blue", "lavender", "warm"].contains(&d.color.as_str())
        || d.icon.len() > 50
        || d.planning_target.is_some_and(|n| !(0..=750).contains(&n))
    {
        return Err(DbError::invalid());
    }
    for value in [&d.start_date, &d.target_date].into_iter().flatten() {
        date(value)?;
    }
    if d.start_date
        .as_ref()
        .zip(d.target_date.as_ref())
        .is_some_and(|(a, b)| a > b)
    {
        return Err(DbError::invalid());
    }
    if kind == "journey" && input.journey_id.is_some() {
        return Err(DbError::invalid());
    }
    if kind == "phase" && input.journey_id.is_none() {
        return Err(DbError::invalid());
    }
    if kind == "phase" && d.progress_mode != "manual" {
        return Err(DbError::invalid());
    }
    if !["journey", "subject"].contains(&kind) && d.parent_id.is_some() {
        return Err(DbError::invalid());
    }
    Ok(())
}
pub(super) async fn save(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    kind: &str,
    revision: Option<i64>,
    input: &EntityInput,
    timestamp: &str,
) -> Result<()> {
    validate_id(id)?;
    validate_entity(kind, input)?;
    if let Some(parent) = &input.journey_id {
        reference(tx, "journey", parent).await?
    }
    if let Some(parent) = &input.data.parent_id {
        reference(
            tx,
            if kind == "subject" {
                "subject"
            } else {
                "journey"
            },
            parent,
        )
        .await?;
        let mut cursor = Some(parent.clone());
        let mut visited = std::collections::BTreeSet::new();
        while let Some(current) = cursor {
            if current == id || !visited.insert(current.clone()) {
                return Err(DbError::invalid());
            }
            let json: Option<String> = sqlx::query_scalar(&format!(
                "SELECT planning_data FROM {} WHERE id=?",
                table(if kind == "subject" {
                    "subject"
                } else {
                    "journey"
                })?
            ))
            .bind(current)
            .fetch_optional(&mut **tx)
            .await?;
            cursor = match json {
                Some(j) => decode::<Details>(&j)?.parent_id,
                None => None,
            };
        }
    }
    let json = serde_json::to_string(&input.data).map_err(|_| DbError::invalid())?;
    let table = table(kind)?;
    if let Some(revision) = revision {
        let parent = if kind == "journey" {
            "target_date=?"
        } else {
            "journey_id=?"
        };
        let change=sqlx::query(&format!("UPDATE {table} SET title=?,planning_data=?,{parent},revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL"))
            .bind(input.title.trim()).bind(json).bind(if kind=="journey"{&input.data.target_date}else{&input.journey_id}).bind(timestamp).bind(id).bind(revision).execute(&mut **tx).await?;
        if change.rows_affected() != 1 {
            return Err(conflict());
        }
    } else {
        let parent = if kind == "journey" {
            "target_date"
        } else {
            "journey_id"
        };
        sqlx::query(&format!("INSERT INTO {table}(id,title,planning_data,{parent},created_at,updated_at) VALUES(?,?,?,?,?,?)")).bind(id).bind(input.title.trim()).bind(json).bind(if kind=="journey"{&input.data.target_date}else{&input.journey_id}).bind(timestamp).bind(timestamp).execute(&mut **tx).await?;
    }
    Ok(())
}
pub(crate) async fn write(
    tx: &mut Transaction<'_, Sqlite>,
    action: &Action,
    timestamp: &str,
) -> Result<()> {
    match action {
        Action::Save {
            id,
            kind,
            revision,
            input,
        } => {
            if revision.is_some() || super::super::receipt(tx, id, action).await? {
                save(tx, id, kind, *revision, input, timestamp).await?
            }
        }
        Action::Lifecycle {
            id,
            kind,
            revision,
            operation,
        } => {
            validate_id(id)?;
            let column = match operation.as_str() {
                "trash" | "restore" => "deleted_at",
                "archive" | "unarchive" => "archived_at",
                _ => return Err(DbError::invalid()),
            };
            let value = if ["trash", "archive"].contains(&operation.as_str()) {
                Some(timestamp)
            } else {
                None
            };
            let r=sqlx::query(&format!("UPDATE {} SET {column}=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?",table(kind)?)).bind(value).bind(timestamp).bind(id).bind(revision).execute(&mut **tx).await?;
            if r.rows_affected() != 1 {
                return Err(conflict());
            }
        }
        Action::Preferences { input: p } => {
            if !(0..=1440).contains(&p.weekday_minutes)
                || !(0..=1440).contains(&p.weekend_minutes)
                || !(0..=10080).contains(&p.side_quest_max)
                || !(0..=750).contains(&p.planning_target)
            {
                return Err(DbError::invalid());
            }
            let revision: i64 = sqlx::query_scalar(
                "SELECT COALESCE((SELECT revision FROM planning_preferences WHERE id=1),0)",
            )
            .fetch_one(&mut **tx)
            .await?;
            if revision != p.revision {
                return Err(conflict());
            }
            sqlx::query("INSERT INTO planning_preferences(id,weekday_minutes,weekend_minutes,side_quest_max,planning_target,revision,updated_at) VALUES(1,?,?,?,?,1,?) ON CONFLICT(id) DO UPDATE SET weekday_minutes=excluded.weekday_minutes,weekend_minutes=excluded.weekend_minutes,side_quest_max=excluded.side_quest_max,planning_target=excluded.planning_target,revision=planning_preferences.revision+1,updated_at=excluded.updated_at").bind(p.weekday_minutes).bind(p.weekend_minutes).bind(p.side_quest_max).bind(p.planning_target).bind(timestamp).execute(&mut **tx).await?;
        }
        Action::Capacity { input: d } => {
            date(&d.date)?;
            if !(0..=1440).contains(&d.minutes)
                || ![
                    "normal",
                    "rest",
                    "exam",
                    "competition",
                    "travel",
                    "sick",
                    "paper_sprint",
                    "other",
                ]
                .contains(&d.status.as_str())
            {
                return Err(DbError::invalid());
            }
            sqlx::query("INSERT INTO capacity_days(date,minutes,status,updated_at) VALUES(?,?,?,?) ON CONFLICT(date) DO UPDATE SET minutes=excluded.minutes,status=excluded.status,updated_at=excluded.updated_at").bind(&d.date).bind(d.minutes).bind(&d.status).bind(timestamp).execute(&mut **tx).await?;
        }
        Action::TaskPlan {
            id,
            revision,
            journey_id,
            subject_id,
            quest_id,
            input: o,
        } => {
            validate_id(id)?;
            super::super::ensure_no_session(tx, id).await?;
            if !(1..=1440).contains(&o.minimum_session)
                || o.preferred_session < o.minimum_session
                || o.preferred_session > 1440
                || !["low", "normal", "high"].contains(&o.energy_level.as_str())
            {
                return Err(DbError::invalid());
            }
            if let Some(d) = &o.earliest_start {
                date(d)?;
            }
            for (kind, parent) in [
                ("journey", journey_id),
                ("subject", subject_id),
                ("quest", quest_id),
            ] {
                if let Some(parent) = parent {
                    reference(tx, kind, parent).await?;
                    if kind != "journey" {
                        let linked: Option<String> = sqlx::query_scalar(&format!(
                            "SELECT journey_id FROM {} WHERE id=?",
                            table(kind)?
                        ))
                        .bind(parent)
                        .fetch_one(&mut **tx)
                        .await?;
                        if linked.is_some() && linked != *journey_id {
                            return Err(DbError::invalid());
                        }
                    }
                }
            }
            let json = serde_json::to_string(o).map_err(|_| DbError::invalid())?;
            let r=sqlx::query("UPDATE tasks SET journey_id=?,subject_id=?,quest_id=?,planning_data=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL AND lifecycle IN ('planned','active')").bind(journey_id).bind(subject_id).bind(quest_id).bind(json).bind(timestamp).bind(id).bind(revision).execute(&mut **tx).await?;
            if r.rows_affected() != 1 {
                return Err(conflict());
            }
        }
        Action::Rule { id, input: r } => {
            validate_id(id)?;
            if !(2000..=2100).contains(&r.year)
                || r.school.trim().is_empty()
                || r.school.len() > 1000
                || r.college.len() > 1000
                || r.major.len() > 1000
                || r.body.trim().is_empty()
                || r.body.len() > 100000
                || r.notes.len() > 10000
                || r.written_exam.len() > 10000
                || r.interview.len() > 10000
                || !(r.source_url.starts_with("https://gsao.fudan.edu.cn/")
                    || r.source_url.starts_with("https://gsas.fudan.edu.cn/")
                    || r.source_url.starts_with("https://www.fudan.edu.cn/")
                    || r.source_url.starts_with("https://yz.chsi.com.cn/"))
                || r.source_url.len() > 2048
            {
                return Err(DbError::invalid());
            }
            if let Some(parent) = &r.journey_id {
                reference(tx, "journey", parent).await?
            }
            if super::super::receipt(tx, id, action).await? {
                sqlx::query("INSERT INTO exam_rule_snapshots(id,journey_id,school,college,major,year,written_exam,interview,source_url,body,notes,content_hash,captured_at,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)").bind(id).bind(&r.journey_id).bind(&r.school).bind(&r.college).bind(&r.major).bind(r.year).bind(&r.written_exam).bind(&r.interview).bind(&r.source_url).bind(&r.body).bind(&r.notes).bind(checksum(r.body.as_bytes())).bind(timestamp).bind(timestamp).bind(timestamp).execute(&mut **tx).await?;
            }
        }
        Action::RuleTrash { id, deleted } => {
            validate_id(id)?;
            let r =
                sqlx::query("UPDATE exam_rule_snapshots SET deleted_at=?,updated_at=? WHERE id=?")
                    .bind(if *deleted { Some(timestamp) } else { None })
                    .bind(timestamp)
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            if r.rows_affected() != 1 {
                return Err(conflict());
            }
        }
        _ => return Err(DbError::invalid()),
    }
    Ok(())
}
