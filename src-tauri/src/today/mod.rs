use crate::database::{checksum, now, repository::validate_id, Database, DbError, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, Transaction};
use std::{
    fs::{File, OpenOptions},
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    sync::Arc,
    time::Instant,
};
use tokio::sync::Mutex;
pub mod calendar;
pub(crate) mod intervals;
pub mod native_events;
#[cfg(test)]
mod phase5_tests;
pub mod roadmap;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskInput {
    pub title: String,
    pub body: String,
    pub due_date: Option<String>,
    pub estimated_minutes: i64,
    pub priority: i64,
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Create {
        id: String,
        input: TaskInput,
    },
    Edit {
        id: String,
        revision: i64,
        input: TaskInput,
    },
    TaskState {
        id: String,
        revision: i64,
        state: String,
    },
    Trash {
        id: String,
        revision: i64,
        deleted: bool,
    },
    Start {
        id: String,
        task_id: String,
    },
    Session {
        id: String,
        operation: String,
    },
    Capture {
        id: String,
        body: String,
    },
    Capacity {
        minutes: i64,
    },
    CalendarCreate {
        id: String,
        input: calendar::EventInput,
    },
    CalendarEdit {
        id: String,
        revision: i64,
        input: calendar::EventInput,
    },
    CalendarTrash {
        id: String,
        revision: i64,
        deleted: bool,
    },
}
#[derive(Debug, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub body: String,
    pub due_date: Option<String>,
    pub estimated_minutes: i64,
    pub remaining_minutes: i64,
    pub priority: i64,
    pub lifecycle: String,
    pub revision: i64,
    pub deleted_at: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Session {
    pub id: String,
    pub task_id: String,
    pub state: String,
    pub elapsed_ms: i64,
    pub paused_ms: i64,
    pub interrupted_ms: i64,
    pub planned_seconds: i64,
    pub last_activity_at: String,
    pub interruption_reason: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Note {
    pub id: String,
    pub body: String,
}
#[derive(Debug, Serialize)]
pub struct TimelineItem {
    pub id: String,
    pub title: String,
    pub started_at: String,
    pub actual_seconds: i64,
}
#[derive(Serialize)]
pub struct Snapshot {
    pub date: String,
    pub tasks: Vec<Task>,
    pub trash: Vec<Task>,
    pub session: Option<Session>,
    pub capacity_minutes: i64,
    pub planned_minutes: i64,
    pub actual_seconds: i64,
    pub journey: Option<String>,
    pub notes: Vec<Note>,
    pub timeline: Vec<TimelineItem>,
    pub system_blocked: bool,
    pub native_hooks_ready: bool,
}
struct Clock {
    instant: Instant,
    wall: DateTime<Utc>,
    system_epoch: u64,
    write_failed: bool,
}
pub struct TodayService {
    pub(crate) db: Database,
    clock: Mutex<Clock>,
    _writer_lock: File,
    pub(crate) system_epoch: AtomicU64,
    pub(crate) system_mask: AtomicU32,
    pub(crate) system_reason: AtomicU32,
    pub native_hooks_ready: AtomicBool,
}
fn conflict() -> DbError {
    DbError::new(
        "STATE_CONFLICT",
        "记录已变化或当前状态不允许此操作，请刷新后重试。",
    )
}
fn validate(input: &TaskInput) -> Result<()> {
    if input.title.trim().is_empty()
        || input.title.len() > 1000
        || input.body.len() > 1_000_000
        || !(1..=1440).contains(&input.estimated_minutes)
        || !(0..=3).contains(&input.priority)
    {
        return Err(DbError::invalid());
    }
    if let Some(date) = &input.due_date {
        let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
        if parsed.format("%Y-%m-%d").to_string() != *date || date.len() != 10 {
            return Err(DbError::invalid());
        }
    }
    Ok(())
}
fn task(row: sqlx::sqlite::SqliteRow) -> Task {
    Task {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        due_date: row.get("due_date"),
        estimated_minutes: row.get("estimated_minutes"),
        remaining_minutes: row.get("remaining_minutes"),
        priority: row.get("priority"),
        lifecycle: row.get("lifecycle"),
        revision: row.get("revision"),
        deleted_at: row.get("deleted_at"),
    }
}
fn session(row: sqlx::sqlite::SqliteRow) -> Session {
    Session {
        id: row.get("id"),
        task_id: row.get("task_id"),
        state: row.get("state"),
        elapsed_ms: row.get("elapsed_ms"),
        paused_ms: row.get("paused_ms"),
        interrupted_ms: row.get("interrupted_ms"),
        planned_seconds: row.get("planned_seconds"),
        last_activity_at: row.get("last_activity_at"),
        interruption_reason: row.get("interruption_reason"),
    }
}
impl TodayService {
    pub async fn open(db: Database) -> Result<Self> {
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(db.root.join("today-writer.lock"))?;
        lock.try_lock_exclusive().map_err(|_| {
            DbError::new(
                "DATABASE_BUSY",
                "另一个 Lumen 窗口正在使用任务与计时，请在该窗口操作。",
            )
        })?;
        // Never turn offline time into focus time. Recovery remains explicit.
        let timestamp = now();
        sqlx::query("UPDATE focus_sessions SET state='interrupted',interruption_reason='restart',interrupted_ms=interrupted_ms+MAX(0,CAST((julianday(?)-julianday(updated_at))*86400000 AS INTEGER)),updated_at=? WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL")
            .bind(&timestamp).bind(&timestamp).execute(&db.pool).await?;
        Ok(Self {
            db,
            clock: Mutex::new(Clock {
                instant: Instant::now(),
                wall: Utc::now(),
                system_epoch: 0,
                write_failed: false,
            }),
            _writer_lock: lock,
            system_epoch: AtomicU64::new(0),
            system_mask: AtomicU32::new(0),
            system_reason: AtomicU32::new(0),
            native_hooks_ready: AtomicBool::new(false),
        })
    }
    pub fn spawn_checkpoint(service: Arc<Self>) {
        tauri::async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            loop {
                interval.tick().await;
                let mut clock = service.clock.lock().await;
                if service.tick(&mut clock).await.is_err() {
                    // On a write failure don't accumulate the unconfirmed gap on recovery.
                    clock.instant = Instant::now();
                    clock.wall = Utc::now();
                    clock.write_failed = true;
                }
            }
        });
    }
    async fn tick(&self, clock: &mut Clock) -> Result<()> {
        let instant = Instant::now();
        let wall = Utc::now();
        let delta = instant
            .duration_since(clock.instant)
            .as_millis()
            .min(i64::MAX as u128) as i64;
        let wall_delta = (wall - clock.wall).num_milliseconds();
        let epoch = self.system_epoch.load(Ordering::SeqCst);
        let system_event = epoch != clock.system_epoch;
        let gap = system_event
            || clock.write_failed
            || delta > 15_000
            || !(0..=15_000).contains(&wall_delta)
            || (delta - wall_delta).abs() > 3000;
        let timestamp = wall.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mut tx = self.db.pool.begin_with("BEGIN IMMEDIATE").await?;
        if gap {
            let reason = if system_event {
                if self.system_reason.load(Ordering::SeqCst) == 1 {
                    "screen_lock"
                } else {
                    "sleep"
                }
            } else {
                "timestamp_gap"
            };
            sqlx::query("UPDATE focus_sessions SET state='interrupted',interruption_reason=?,interrupted_ms=interrupted_ms+?,updated_at=? WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL")
                .bind(reason).bind(wall_delta.max(0)).bind(&timestamp).execute(&mut *tx).await?;
        } else {
            if let Some(row)=sqlx::query("SELECT id,segment_no FROM focus_sessions WHERE state='running' AND deleted_at IS NULL").fetch_optional(&mut *tx).await? {
                intervals::record(&mut tx,&row.get::<String,_>("id"),row.get("segment_no"),clock.wall,wall,delta).await?;
            }
            sqlx::query("UPDATE focus_sessions SET elapsed_ms=elapsed_ms+CASE WHEN state='running' THEN ? ELSE 0 END,paused_ms=paused_ms+CASE WHEN state='paused' THEN ? ELSE 0 END,interrupted_ms=interrupted_ms+CASE WHEN state='interrupted' THEN ? ELSE 0 END,last_activity_at=CASE WHEN state='interrupted' THEN last_activity_at ELSE ? END,updated_at=? WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL")
                .bind(delta).bind(delta).bind(delta).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        clock.instant = instant;
        clock.wall = wall;
        clock.system_epoch = epoch;
        clock.write_failed = false;
        Ok(())
    }
    pub async fn snapshot(&self) -> Result<Snapshot> {
        let mut clock = self.clock.lock().await;
        self.tick(&mut clock).await?;
        self.read().await
    }
    async fn read(&self) -> Result<Snapshot> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let mut tx = self.db.pool.begin().await?;
        let tasks=sqlx::query("SELECT * FROM tasks WHERE deleted_at IS NULL AND (lifecycle='active' OR (lifecycle='planned' AND (due_date IS NULL OR due_date<=? OR EXISTS(SELECT 1 FROM planning_blocks b WHERE b.task_id=tasks.id AND b.date=?))) OR (lifecycle NOT IN ('active','planned') AND date(updated_at,'localtime')=?)) ORDER BY CASE lifecycle WHEN 'active' THEN 0 WHEN 'planned' THEN 1 ELSE 2 END,priority DESC,due_date,created_at,id")
            .bind(&date).bind(&date).bind(&date).fetch_all(&mut *tx).await?.into_iter().map(task).collect::<Vec<_>>();
        let trash = sqlx::query(
            "SELECT * FROM tasks WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC LIMIT 100",
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(task)
        .collect();
        let active=sqlx::query("SELECT * FROM focus_sessions WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL").fetch_optional(&mut *tx).await?.map(session);
        let capacity: Option<String>=sqlx::query_scalar("SELECT value_text FROM settings WHERE key='daily_capacity_minutes' AND deleted_at IS NULL").fetch_optional(&mut *tx).await?;
        let fallback_capacity = capacity
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|v| (0..=1440).contains(v))
            .unwrap_or(210);
        let weekly_default: Option<i64> =
            sqlx::query_scalar(if Local::now().weekday().number_from_monday() <= 5 {
                "SELECT weekday_minutes FROM planning_preferences WHERE id=1"
            } else {
                "SELECT weekend_minutes FROM planning_preferences WHERE id=1"
            })
            .fetch_optional(&mut *tx)
            .await?;
        let capacity_minutes =
            sqlx::query_scalar::<_, i64>("SELECT minutes FROM capacity_days WHERE date=?")
                .bind(&date)
                .fetch_optional(&mut *tx)
                .await?
                .unwrap_or(weekly_default.unwrap_or(fallback_capacity));
        let planned_blocks: Vec<(String, i64)> = sqlx::query_as(
            "SELECT task_id,SUM(minutes) FROM planning_blocks WHERE date=? GROUP BY task_id",
        )
        .bind(&date)
        .fetch_all(&mut *tx)
        .await?;
        let actual_seconds: i64=sqlx::query_scalar("SELECT CAST(COALESCE(SUM(value),0) AS INTEGER) FROM daily_metrics WHERE metric_date=? AND metric_key='focus_seconds' AND deleted_at IS NULL").bind(&date).fetch_one(&mut *tx).await?;
        let current_task = tasks
            .iter()
            .find(|t| matches!(t.lifecycle.as_str(), "active" | "planned"));
        let journey=sqlx::query_scalar("SELECT j.title FROM journeys j JOIN tasks t ON t.journey_id=j.id WHERE t.id=? AND j.deleted_at IS NULL").bind(current_task.map(|t| &t.id)).fetch_optional(&mut *tx).await?;
        let notes=sqlx::query("SELECT id,body FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0 AND entry_date=? ORDER BY created_at DESC LIMIT 3").bind(&date).fetch_all(&mut *tx).await?.into_iter().map(|r|Note{id:r.get("id"),body:r.get("body")}).collect();
        let timeline=sqlx::query("SELECT f.id,t.title,MIN(i.started_at) AS started_at,CAST(SUM(i.elapsed_ms)/1000 AS INTEGER) AS actual_seconds FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id JOIN tasks t ON t.id=f.task_id WHERE i.local_date=? AND f.state='finished' AND f.deleted_at IS NULL GROUP BY f.id ORDER BY f.started_at DESC LIMIT 10").bind(&date).fetch_all(&mut *tx).await?.into_iter().map(|r|TimelineItem{id:r.get("id"),title:r.get("title"),started_at:r.get("started_at"),actual_seconds:r.get("actual_seconds")}).collect();
        tx.commit().await?;
        let planned_minutes = tasks
            .iter()
            .filter(|t| !matches!(t.lifecycle.as_str(), "skipped" | "cancelled" | "moved"))
            .map(|t| {
                planned_blocks
                    .iter()
                    .find(|(id, _)| id == &t.id)
                    .map(|(_, minutes)| *minutes)
                    .unwrap_or(t.estimated_minutes)
            })
            .sum();
        Ok(Snapshot {
            date,
            tasks,
            trash,
            session: active,
            capacity_minutes,
            planned_minutes,
            actual_seconds,
            journey,
            notes,
            timeline,
            system_blocked: self.system_mask.load(Ordering::SeqCst) != 0,
            native_hooks_ready: self.native_hooks_ready.load(Ordering::SeqCst),
        })
    }
    pub async fn act(&self, action: Action) -> Result<Snapshot> {
        let mut clock = self.clock.lock().await;
        self.tick(&mut clock).await?;
        let mut tx = self.db.pool.begin_with("BEGIN IMMEDIATE").await?;
        let timestamp = now();
        match &action {
            Action::Create { id, input } => {
                validate_id(id)?;
                validate(input)?;
                if receipt(&mut tx, id, &action).await? {
                    sqlx::query("INSERT INTO tasks(id,title,body,due_date,estimated_minutes,remaining_minutes,priority,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?)")
                        .bind(id).bind(input.title.trim()).bind(&input.body).bind(&input.due_date).bind(input.estimated_minutes).bind(input.estimated_minutes).bind(input.priority).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
                }
            }
            Action::Edit {
                id,
                revision,
                input,
            } => {
                validate_id(id)?;
                validate(input)?;
                ensure_no_session(&mut tx, id).await?;
                let changed=sqlx::query("UPDATE tasks SET title=?,body=?,due_date=?,estimated_minutes=?,remaining_minutes=MAX(0,?-COALESCE((SELECT SUM(actual_seconds)/60 FROM focus_sessions WHERE task_id=tasks.id AND state='finished'),0)),priority=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL")
                    .bind(input.title.trim()).bind(&input.body).bind(&input.due_date).bind(input.estimated_minutes).bind(input.estimated_minutes).bind(input.priority).bind(&timestamp).bind(id).bind(revision).execute(&mut *tx).await?;
                if changed.rows_affected() != 1 {
                    return Err(conflict());
                }
            }
            Action::TaskState {
                id,
                revision,
                state,
            } => {
                validate_id(id)?;
                ensure_no_session(&mut tx, id).await?;
                if !["planned", "done", "skipped", "moved", "cancelled"].contains(&state.as_str()) {
                    return Err(DbError::invalid());
                }
                let changed=sqlx::query("UPDATE tasks SET lifecycle=?,status=CASE WHEN ?='done' THEN 'done' ELSE 'todo' END,remaining_minutes=CASE WHEN ?='done' THEN 0 ELSE estimated_minutes END,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL")
                    .bind(state).bind(state).bind(state).bind(&timestamp).bind(id).bind(revision).execute(&mut *tx).await?;
                if changed.rows_affected() != 1 {
                    return Err(conflict());
                }
            }
            Action::Trash {
                id,
                revision,
                deleted,
            } => {
                validate_id(id)?;
                ensure_no_session(&mut tx, id).await?;
                let changed=sqlx::query("UPDATE tasks SET deleted_at=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?")
                    .bind(if *deleted{Some(&timestamp)}else{None}).bind(&timestamp).bind(id).bind(revision).execute(&mut *tx).await?;
                if changed.rows_affected() != 1 {
                    return Err(conflict());
                }
            }
            Action::Start { id, task_id } => {
                if self.system_mask.load(Ordering::SeqCst) != 0 {
                    return Err(conflict());
                }
                validate_id(id)?;
                validate_id(task_id)?;
                if receipt(&mut tx, id, &action).await? {
                    let estimate: Option<i64>=sqlx::query_scalar("SELECT estimated_minutes FROM tasks WHERE id=? AND lifecycle IN ('planned','active') AND deleted_at IS NULL").bind(task_id).fetch_optional(&mut *tx).await?;
                    let minutes = estimate.ok_or_else(conflict)?;
                    let count:i64=sqlx::query_scalar("SELECT count(*) FROM focus_sessions WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL").fetch_one(&mut *tx).await?;
                    if count != 0 {
                        return Err(conflict());
                    }
                    sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,planned_seconds,state,last_activity_at,session_date,created_at,updated_at) VALUES(?,?,?,?,'running',?,?,?,?)")
                        .bind(id).bind(task_id).bind(&timestamp).bind(minutes*60).bind(&timestamp).bind(Local::now().format("%Y-%m-%d").to_string()).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
                    sqlx::query("UPDATE tasks SET lifecycle='active',status='doing',revision=revision+1,updated_at=? WHERE id=?").bind(&timestamp).bind(task_id).execute(&mut *tx).await?;
                }
            }
            Action::Session { id, operation } => {
                if operation == "resume" && self.system_mask.load(Ordering::SeqCst) != 0 {
                    return Err(conflict());
                }
                validate_id(id)?;
                let row =
                    sqlx::query("SELECT * FROM focus_sessions WHERE id=? AND deleted_at IS NULL")
                        .bind(id)
                        .fetch_optional(&mut *tx)
                        .await?
                        .ok_or_else(conflict)?;
                let current: String = row.get("state");
                let task_id: String = row.get("task_id");
                let next = match (operation.as_str(), current.as_str()) {
                    ("pause", "running" | "paused") => "paused",
                    ("resume", "paused" | "interrupted" | "running") => "running",
                    ("interrupt", "running" | "paused" | "interrupted") => "interrupted",
                    ("finish", "running" | "paused" | "interrupted" | "finished") => "finished",
                    ("cancel", "running" | "paused" | "interrupted" | "cancelled") => "cancelled",
                    _ => return Err(conflict()),
                };
                if next != current {
                    if ["finished", "cancelled"].contains(&next) {
                        let elapsed: i64 = row.get("elapsed_ms");
                        let end = if current == "interrupted" {
                            row.get::<String, _>("last_activity_at")
                        } else {
                            timestamp.clone()
                        };
                        sqlx::query("UPDATE focus_sessions SET state=?,ended_at=?,actual_seconds=?,updated_at=? WHERE id=?").bind(next).bind(end).bind(elapsed/1000).bind(&timestamp).bind(id).execute(&mut *tx).await?;
                        sqlx::query("UPDATE tasks SET lifecycle=?,status=?,remaining_minutes=CASE WHEN ?='finished' THEN 0 ELSE estimated_minutes END,revision=revision+1,updated_at=? WHERE id=?")
                            .bind(if next=="finished"{"done"}else{"planned"}).bind(if next=="finished"{"done"}else{"todo"}).bind(next).bind(&timestamp).bind(&task_id).execute(&mut *tx).await?;
                        if next == "finished" {
                            intervals::rebuild_metrics(&mut tx, id, &timestamp).await?;
                        }
                    } else {
                        sqlx::query("UPDATE focus_sessions SET state=?,segment_no=segment_no+CASE WHEN ?='running' THEN 1 ELSE 0 END,interruption_reason=CASE WHEN ?='interrupted' THEN 'manual' ELSE NULL END,last_activity_at=?,updated_at=? WHERE id=?").bind(next).bind(next).bind(next).bind(&timestamp).bind(&timestamp).bind(id).execute(&mut *tx).await?;
                    }
                }
            }
            Action::Capture { id, body } => {
                validate_id(id)?;
                if body.trim().is_empty() || body.len() > 1_000_000 {
                    return Err(DbError::invalid());
                }
                if receipt(&mut tx, id, &action).await? {
                    let title: String = body.trim().chars().take(60).collect();
                    sqlx::query("INSERT INTO journal_entries(id,title,body,entry_date,created_at,updated_at,entry_type) VALUES(?,?,?,?,?,?,'quick_note')").bind(id).bind(title).bind(body.trim()).bind(Local::now().format("%Y-%m-%d").to_string()).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
                }
            }
            Action::Capacity { minutes } => {
                if !(0..=1440).contains(minutes) {
                    return Err(DbError::invalid());
                }
                sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'daily_capacity_minutes',?,?,?) ON CONFLICT(key) DO UPDATE SET value_text=excluded.value_text,updated_at=excluded.updated_at,deleted_at=NULL")
                    .bind(uuid::Uuid::new_v4().to_string()).bind(minutes.to_string()).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
                sqlx::query("INSERT INTO capacity_days(date,minutes,status,updated_at) VALUES(?,?,'normal',?) ON CONFLICT(date) DO UPDATE SET minutes=excluded.minutes,updated_at=excluded.updated_at").bind(Local::now().date_naive().to_string()).bind(minutes).bind(&timestamp).execute(&mut *tx).await?;
            }
            Action::CalendarCreate { .. }
            | Action::CalendarEdit { .. }
            | Action::CalendarTrash { .. } => calendar::write(&mut tx, &action, &timestamp).await?,
        }
        tx.commit().await?;
        clock.instant = Instant::now();
        clock.wall = Utc::now();
        self.read().await
    }
}
async fn receipt<T: Serialize>(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    action: &T,
) -> Result<bool> {
    let digest = checksum(&serde_json::to_vec(action).map_err(|_| DbError::invalid())?);
    let existing: Option<String> =
        sqlx::query_scalar("SELECT payload_sha256 FROM request_receipts WHERE id=?")
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
    if let Some(existing) = existing {
        if existing != digest {
            return Err(conflict());
        }
        return Ok(false);
    }
    sqlx::query("INSERT INTO request_receipts(id,payload_sha256,created_at) VALUES(?,?,?)")
        .bind(id)
        .bind(digest)
        .bind(now())
        .execute(&mut **tx)
        .await?;
    Ok(true)
}
async fn ensure_no_session(tx: &mut Transaction<'_, Sqlite>, id: &str) -> Result<()> {
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM focus_sessions WHERE task_id=? AND state IN ('running','paused','interrupted') AND deleted_at IS NULL").bind(id).fetch_one(&mut **tx).await?;
    if count > 0 {
        return Err(conflict());
    }
    Ok(())
}
