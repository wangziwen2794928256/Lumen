use super::*;
use chrono::TimeZone;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EventInput {
    pub title: String,
    pub event_type: String,
    pub event_date: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub task_id: Option<String>,
}
#[derive(Serialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub event_type: String,
    pub event_date: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub task_id: Option<String>,
    pub revision: i64,
}
#[derive(Serialize)]
pub struct ActualInterval {
    pub id: String,
    pub session_id: String,
    pub task_id: Option<String>,
    pub title: String,
    pub local_date: String,
    pub starts_at: String,
    pub ends_at: String,
    pub elapsed_ms: i64,
    pub state: String,
    pub provenance: String,
}
#[derive(Serialize)]
pub struct CalendarSnapshot {
    pub from: String,
    pub until: String,
    pub events: Vec<CalendarEvent>,
    pub actual: Vec<ActualInterval>,
    pub tasks: Vec<Task>,
    pub task_options: Vec<Task>,
    pub trash: Vec<CalendarEvent>,
}
fn date(value: &str) -> Result<NaiveDate> {
    let d = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
    if d.to_string() != value {
        return Err(DbError::invalid());
    }
    Ok(d)
}
pub(super) fn validate_event(input: &EventInput) -> Result<EventInput> {
    if input.title.trim().is_empty()
        || input.title.len() > 1000
        || ![
            "study",
            "class",
            "competition",
            "exam",
            "deadline",
            "paper",
            "personal",
        ]
        .contains(&input.event_type.as_str())
    {
        return Err(DbError::invalid());
    }
    if let Some(id) = &input.task_id {
        validate_id(id)?;
    }
    let mut normalized = input.clone();
    normalized.title = input.title.trim().into();
    match (&input.starts_at, &input.ends_at) {
        (Some(start), Some(end)) => {
            let start = DateTime::parse_from_rfc3339(start)
                .map_err(|_| DbError::invalid())?
                .with_timezone(&Utc);
            let end = DateTime::parse_from_rfc3339(end)
                .map_err(|_| DbError::invalid())?
                .with_timezone(&Utc);
            if end <= start
                || (end - start) > chrono::Duration::days(7)
                || input.event_date.is_some()
            {
                return Err(DbError::invalid());
            }
            normalized.starts_at = Some(start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
            normalized.ends_at = Some(end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
        }
        (None, None) => {
            date(input.event_date.as_deref().ok_or_else(DbError::invalid)?)?;
        }
        _ => return Err(DbError::invalid()),
    }
    Ok(normalized)
}
fn event(row: sqlx::sqlite::SqliteRow) -> CalendarEvent {
    CalendarEvent {
        id: row.get("id"),
        title: row.get("title"),
        event_type: row.get("event_type"),
        event_date: row.get("event_date"),
        starts_at: row.get("starts_at"),
        ends_at: row.get("ends_at"),
        task_id: row.get("task_id"),
        revision: row.get("revision"),
    }
}
pub(super) async fn write(
    tx: &mut Transaction<'_, Sqlite>,
    action: &Action,
    timestamp: &str,
) -> Result<()> {
    match action {
        Action::CalendarCreate { id, input } | Action::CalendarEdit { id, input, .. } => {
            validate_id(id)?;
            let input = validate_event(input)?;
            if let Some(task_id) = &input.task_id {
                let found: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM tasks WHERE id=? AND deleted_at IS NULL",
                )
                .bind(task_id)
                .fetch_one(&mut **tx)
                .await?;
                if found != 1 {
                    return Err(conflict());
                }
            }
            if let Action::CalendarEdit { revision, .. } = action {
                let result=sqlx::query("UPDATE calendar_events SET title=?,event_type=?,event_date=?,starts_at=?,ends_at=?,task_id=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL")
                    .bind(input.title).bind(input.event_type).bind(input.event_date).bind(input.starts_at).bind(input.ends_at).bind(input.task_id).bind(timestamp).bind(id).bind(revision).execute(&mut **tx).await?;
                if result.rows_affected() != 1 {
                    return Err(conflict());
                }
            } else if receipt(tx, id, action).await? {
                sqlx::query("INSERT INTO calendar_events(id,title,event_type,event_date,starts_at,ends_at,task_id,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?)")
                    .bind(id).bind(input.title).bind(input.event_type).bind(input.event_date).bind(input.starts_at).bind(input.ends_at).bind(input.task_id).bind(timestamp).bind(timestamp).execute(&mut **tx).await?;
            }
        }
        Action::CalendarTrash {
            id,
            revision,
            deleted,
        } => {
            validate_id(id)?;
            let result=sqlx::query("UPDATE calendar_events SET deleted_at=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?")
                .bind(if *deleted{Some(timestamp)}else{None}).bind(timestamp).bind(id).bind(revision).execute(&mut **tx).await?;
            if result.rows_affected() != 1 {
                return Err(conflict());
            }
        }
        _ => return Err(DbError::invalid()),
    }
    Ok(())
}
impl TodayService {
    pub async fn calendar_snapshot(&self, from: &str, until: &str) -> Result<CalendarSnapshot> {
        let first = date(from)?;
        let last = date(until)?;
        if last <= first || (last - first).num_days() > 62 {
            return Err(DbError::invalid());
        }
        let midnight = |d: NaiveDate| -> Result<String> {
            Ok(Local
                .from_local_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                .earliest()
                .ok_or_else(DbError::invalid)?
                .with_timezone(&Utc)
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        };
        let begin = midnight(first)?;
        let end = midnight(last)?;
        let mut clock = self.clock.lock().await;
        self.tick(&mut clock).await?;
        let mut tx = self.db.pool.begin().await?;
        let events=sqlx::query("SELECT * FROM calendar_events WHERE deleted_at IS NULL AND archived_at IS NULL AND ((event_date>=? AND event_date<?) OR (starts_at<? AND ends_at>?)) ORDER BY COALESCE(starts_at,event_date),id")
            .bind(from).bind(until).bind(&end).bind(&begin).fetch_all(&mut *tx).await?.into_iter().map(event).collect();
        let actual=sqlx::query("SELECT i.*,f.task_id,f.state,COALESCE(t.title,'未关联任务的专注') AS title FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id LEFT JOIN tasks t ON t.id=f.task_id WHERE i.local_date>=? AND i.local_date<? AND f.state<>'cancelled' AND f.deleted_at IS NULL ORDER BY i.started_at,i.segment_no")
            .bind(from).bind(until).fetch_all(&mut *tx).await?.into_iter().map(|r| ActualInterval{id:format!("{}-{}-{}",r.get::<String,_>("session_id"),r.get::<i64,_>("segment_no"),r.get::<String,_>("local_date")),session_id:r.get("session_id"),task_id:r.get("task_id"),title:r.get("title"),local_date:r.get("local_date"),starts_at:r.get("started_at"),ends_at:r.get("ended_at"),elapsed_ms:r.get("elapsed_ms"),state:r.get("state"),provenance:r.get("provenance")}).collect();
        let tasks=sqlx::query("SELECT * FROM tasks WHERE deleted_at IS NULL AND due_date>=? AND due_date<? AND lifecycle NOT IN ('skipped','moved','cancelled') ORDER BY due_date,priority DESC,id")
            .bind(from).bind(until).fetch_all(&mut *tx).await?.into_iter().map(task).collect();
        let task_options=sqlx::query("SELECT * FROM tasks WHERE deleted_at IS NULL AND lifecycle IN ('planned','active') ORDER BY priority DESC,created_at DESC LIMIT 200").fetch_all(&mut *tx).await?.into_iter().map(task).collect();
        let trash=sqlx::query("SELECT * FROM calendar_events WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC LIMIT 100").fetch_all(&mut *tx).await?.into_iter().map(event).collect();
        tx.commit().await?;
        Ok(CalendarSnapshot {
            from: from.into(),
            until: until.into(),
            events,
            actual,
            tasks,
            task_options,
            trash,
        })
    }
}
