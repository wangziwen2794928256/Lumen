#[cfg(test)]
mod tests;
mod tracker;
use crate::database::{checksum, now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::{Arc, Mutex};
#[derive(Clone, Default, Serialize)]
pub struct MediaStatus {
    pub state: String,
    pub metadata_available: bool,
    pub source_app: String,
    pub error: Option<String>,
}
pub struct MediaState(pub Arc<Mutex<MediaStatus>>);
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub enabled: bool,
    pub threshold_seconds: i64,
    pub allowed_app: String,
    pub revision: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Save {
        request_id: String,
        id: Option<String>,
        revision: i64,
        title: String,
        artist: String,
        album: String,
        platform: String,
        played_at: String,
        duration_seconds: i64,
        journal_id: Option<String>,
    },
    Trash {
        request_id: String,
        id: String,
        revision: i64,
        restore: bool,
    },
    Preferences {
        request_id: String,
        preferences: Preferences,
    },
}
fn invalid() -> DbError {
    DbError::invalid()
}
fn conflict() -> DbError {
    DbError::new("MUSIC_CONFLICT", "记录已变化，请重新载入；输入已保留。")
}
fn text(s: &str, required: bool) -> Result<()> {
    if s.chars().count() > 300
        || s.chars().any(char::is_control)
        || (required && s.trim().is_empty())
    {
        Err(invalid())
    } else {
        Ok(())
    }
}
impl Database {
    pub async fn music_preferences(&self) -> Result<Preferences> {
        let r=sqlx::query("SELECT enabled,threshold_seconds,allowed_app,revision FROM music_preferences WHERE singleton=1").fetch_optional(&self.pool).await?;
        Ok(r.map(|r| Preferences {
            enabled: r.get::<i64, _>("enabled") == 1,
            threshold_seconds: r.get("threshold_seconds"),
            allowed_app: r.get("allowed_app"),
            revision: r.get("revision"),
        })
        .unwrap_or(Preferences {
            enabled: false,
            threshold_seconds: 30,
            allowed_app: String::new(),
            revision: 0,
        }))
    }
    pub async fn music_snapshot(
        &self,
        from: String,
        until: String,
        trash: bool,
    ) -> Result<serde_json::Value> {
        let start = chrono::NaiveDate::parse_from_str(&from, "%Y-%m-%d").map_err(|_| invalid())?;
        let end = chrono::NaiveDate::parse_from_str(&until, "%Y-%m-%d").map_err(|_| invalid())?;
        if start.to_string() != from
            || end.to_string() != until
            || end < start
            || (end - start).num_days() > 366
        {
            return Err(invalid());
        }
        let rows=sqlx::query("SELECT p.*,t.title,t.artist,t.album,t.platform,j.title journal_title FROM music_plays p JOIN music_tracks t ON t.id=p.track_id LEFT JOIN journal_entries j ON j.id=p.journal_id AND j.deleted_at IS NULL WHERE (p.deleted_at IS NOT NULL)=? AND date(p.played_at,'localtime') BETWEEN ? AND ? ORDER BY p.played_at DESC,p.id LIMIT 100").bind(trash).bind(&from).bind(&until).fetch_all(&self.pool).await?;
        let items:Vec<_>=rows.iter().map(|r|serde_json::json!({"id":r.get::<String,_>("id"),"title":r.get::<String,_>("title"),"artist":r.get::<String,_>("artist"),"album":r.get::<String,_>("album"),"platform":r.get::<String,_>("platform"),"played_at":r.get::<String,_>("played_at"),"duration_seconds":r.get::<Option<i64>,_>("duration_seconds").unwrap_or(0),"source":r.get::<String,_>("source"),"source_app":r.get::<String,_>("source_app"),"revision":r.get::<i64,_>("revision"),"journal_id":r.get::<Option<String>,_>("journal_id"),"journal_title":r.get::<Option<String>,_>("journal_title")})).collect();
        let cloud=sqlx::query("SELECT t.title,t.artist,t.album,count(*) count,coalesce(sum(p.duration_seconds),0) seconds FROM music_plays p JOIN music_tracks t ON t.id=p.track_id WHERE p.deleted_at IS NULL AND t.deleted_at IS NULL AND date(p.played_at,'localtime') BETWEEN ? AND ? GROUP BY t.title,t.artist,t.album ORDER BY count DESC,t.title LIMIT 500").bind(from).bind(until).fetch_all(&self.pool).await?;
        let groups:Vec<_>=cloud.iter().map(|r|serde_json::json!({"title":r.get::<String,_>("title"),"artist":r.get::<String,_>("artist"),"album":r.get::<String,_>("album"),"count":r.get::<i64,_>("count"),"seconds":r.get::<i64,_>("seconds")})).collect();
        let journals=sqlx::query("SELECT id,title FROM journal_entries WHERE deleted_at IS NULL ORDER BY updated_at DESC LIMIT 100").fetch_all(&self.pool).await?;
        let totals: (i64,i64)=sqlx::query_as("SELECT count(*),coalesce(sum(p.duration_seconds),0) FROM music_plays p JOIN music_tracks t ON t.id=p.track_id WHERE p.deleted_at IS NULL AND t.deleted_at IS NULL AND date(p.played_at,'localtime') BETWEEN ? AND ?").bind(start.to_string()).bind(end.to_string()).fetch_one(&self.pool).await?;
        Ok(
            serde_json::json!({"items":items,"cloud":groups,"preferences":self.music_preferences().await?,"totals":{"count":totals.0,"seconds":totals.1},"journals":journals.iter().map(|r|serde_json::json!({"id":r.get::<String,_>("id"),"title":r.get::<String,_>("title")})).collect::<Vec<_>>(),"visibility":"private"}),
        )
    }
    pub async fn music_action(&self, action: Action) -> Result<()> {
        let (Action::Save { request_id, .. }
        | Action::Trash { request_id, .. }
        | Action::Preferences { request_id, .. }) = &action;
        crate::database::repository::validate_id(request_id)?;
        let hash = checksum(
            serde_json::to_string(&action)
                .map_err(|_| invalid())?
                .as_bytes(),
        );
        let mut tx = self.pool.begin().await?;
        let existing: Option<String> =
            sqlx::query_scalar("SELECT payload_hash FROM music_receipts WHERE request_id=?")
                .bind(request_id)
                .fetch_optional(&mut *tx)
                .await?;
        if let Some(h) = existing {
            return if h == hash { Ok(()) } else { Err(conflict()) };
        }
        match &action {
            Action::Save {
                id,
                revision,
                title,
                artist,
                album,
                platform,
                played_at,
                duration_seconds,
                journal_id,
                ..
            } => {
                text(title, true)?;
                text(artist, false)?;
                text(album, false)?;
                text(platform, false)?;
                if !(0..=86400).contains(duration_seconds)
                    || !played_at.ends_with('Z')
                    || chrono::DateTime::parse_from_rfc3339(played_at).is_err()
                {
                    return Err(invalid());
                }
                if let Some(j) = journal_id {
                    crate::database::repository::validate_id(j)?;
                    let found: i64 = sqlx::query_scalar(
                        "SELECT count(*) FROM journal_entries WHERE id=? AND deleted_at IS NULL",
                    )
                    .bind(j)
                    .fetch_one(&mut *tx)
                    .await?;
                    if found != 1 {
                        return Err(invalid());
                    }
                }
                let track = uuid::Uuid::new_v4().to_string();
                let stamp = now();
                sqlx::query("INSERT INTO music_tracks(id,title,artist,album,platform,created_at,updated_at) VALUES(?,?,?,?,?,?,?)").bind(&track).bind(title.trim()).bind(artist.trim()).bind(album.trim()).bind(platform.trim()).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
                if let Some(id) = id {
                    crate::database::repository::validate_id(id)?;
                    let n=sqlx::query("UPDATE music_plays SET track_id=?,played_at=?,duration_seconds=?,journal_id=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL AND source='manual'").bind(track).bind(played_at).bind(duration_seconds).bind(journal_id).bind(stamp).bind(id).bind(revision).execute(&mut *tx).await?.rows_affected();
                    if n != 1 {
                        return Err(conflict());
                    }
                } else {
                    if *revision != 0 {
                        return Err(invalid());
                    }
                    sqlx::query("INSERT INTO music_plays(id,track_id,played_at,duration_seconds,journal_id,created_at,updated_at) VALUES(?,?,?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(track).bind(played_at).bind(duration_seconds).bind(journal_id).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
                }
            }
            Action::Trash {
                id,
                revision,
                restore,
                ..
            } => {
                crate::database::repository::validate_id(id)?;
                let n=sqlx::query("UPDATE music_plays SET deleted_at=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND (deleted_at IS NOT NULL)=?").bind(if *restore {None}else{Some(now())}).bind(now()).bind(id).bind(revision).bind(restore).execute(&mut *tx).await?.rows_affected();
                if n != 1 {
                    return Err(conflict());
                }
            }
            Action::Preferences { preferences: p, .. } => {
                text(&p.allowed_app, false)?;
                if !(5..=600).contains(&p.threshold_seconds)
                    || (p.enabled && p.allowed_app.trim().is_empty())
                {
                    return Err(invalid());
                }
                let current: Option<i64> =
                    sqlx::query_scalar("SELECT revision FROM music_preferences WHERE singleton=1")
                        .fetch_optional(&mut *tx)
                        .await?;
                if current.unwrap_or(0) != p.revision {
                    return Err(conflict());
                }
                sqlx::query("INSERT INTO music_preferences(singleton,enabled,threshold_seconds,allowed_app,revision,updated_at) VALUES(1,?,?,?,?,?) ON CONFLICT(singleton) DO UPDATE SET enabled=excluded.enabled,threshold_seconds=excluded.threshold_seconds,allowed_app=excluded.allowed_app,revision=excluded.revision,updated_at=excluded.updated_at").bind(p.enabled).bind(p.threshold_seconds).bind(p.allowed_app.trim()).bind(p.revision+1).bind(now()).execute(&mut *tx).await?;
            }
        }
        sqlx::query("INSERT INTO music_receipts VALUES(?,?,?)")
            .bind(request_id)
            .bind(hash)
            .bind(now())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
    async fn record_media(&self, r: &tracker::Recording) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let identity = checksum(r.sample.identity().as_bytes());
        let stamp = now();
        sqlx::query("INSERT OR IGNORE INTO music_tracks(id,title,artist,album,platform,identity_key,created_at,updated_at) VALUES(?,?,?,?,'Windows Media',?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(&r.sample.title).bind(&r.sample.artist).bind(&r.sample.album).bind(&identity).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
        let track: String = sqlx::query_scalar(
            "SELECT id FROM music_tracks WHERE identity_key=? AND deleted_at IS NULL",
        )
        .bind(identity)
        .fetch_one(&mut *tx)
        .await?;
        // A trashed event remains deduplicated and is never silently restored by the monitor.
        sqlx::query("INSERT INTO music_plays(id,track_id,played_at,duration_seconds,source,source_app,event_key,created_at,updated_at) VALUES(?,?,?,?,'windows-media',?,?,?,?) ON CONFLICT(event_key) WHERE event_key IS NOT NULL DO UPDATE SET duration_seconds=max(duration_seconds,excluded.duration_seconds),revision=revision+1,updated_at=excluded.updated_at WHERE music_plays.deleted_at IS NULL").bind(uuid::Uuid::new_v4().to_string()).bind(track).bind(&r.started_at).bind(r.seconds).bind(&r.sample.app).bind(&r.event).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
pub fn initialize(app: &tauri::AppHandle, db: Database) {
    use tauri::Manager;
    let status = Arc::new(Mutex::new(MediaStatus {
        state: "disabled".into(),
        ..Default::default()
    }));
    app.manage(MediaState(status.clone()));
    // Dedicated thread: WinRT initialization/properties must never block the UI/startup.
    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(_) => return,
        };
        let mut adapter = None;
        let mut tracker = tracker::Tracker::default();
        let mut last = std::time::Instant::now();
        let mut pref_revision = -1;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut s = MediaStatus {
                state: "disabled".into(),
                ..Default::default()
            };
            let prefs = runtime.block_on(db.music_preferences());
            let enabled = runtime.block_on(db.require_module("lumen.music")).is_ok();
            match prefs {
                Ok(p) if p.enabled && enabled => {
                    if p.revision != pref_revision {
                        tracker.reset();
                        pref_revision = p.revision;
                    }
                    if adapter.is_none() {
                        adapter = crate::kernel::platform::media::connect().ok();
                    }
                    let poll = adapter.as_mut().map(|a| a.poll(&p.allowed_app));
                    match poll {
                        Some(Ok(sample)) => {
                            s.source_app =
                                sample.as_ref().map(|s| s.app.clone()).unwrap_or_default();
                            let approved = sample.filter(|s| s.app == p.allowed_app && s.valid());
                            s.metadata_available = approved.is_some();
                            s.state = if approved.is_some() {
                                "observing"
                            } else {
                                "manual-fallback"
                            }
                            .into();
                            let delta = last.elapsed().as_millis().min(u64::MAX as u128) as u64;
                            if let Some(r) = tracker.observe(approved, delta, p.threshold_seconds) {
                                let still_allowed = runtime
                                    .block_on(db.music_preferences())
                                    .is_ok_and(|current| {
                                        current.enabled
                                            && current.revision == p.revision
                                            && current.allowed_app == r.sample.app
                                    })
                                    && runtime.block_on(db.require_module("lumen.music")).is_ok();
                                if still_allowed {
                                    match runtime.block_on(db.record_media(&r)) {
                                        Ok(()) => tracker.acknowledge(),
                                        Err(_) => {
                                            s.error = Some("记录未保存，请使用手动模式。".into())
                                        }
                                    }
                                } else {
                                    tracker.reset();
                                }
                            }
                        }
                        _ => {
                            s.state = "manual-fallback".into();
                            s.error = Some("媒体不可用或权限不足；手动记录可继续使用。".into());
                            tracker.reset();
                            adapter = None;
                        }
                    }
                }
                Ok(_) => {
                    tracker.reset();
                    adapter = None;
                }
                Err(_) => {
                    s.state = "manual-fallback".into();
                    s.error = Some("监听设置不可用。".into());
                    tracker.reset();
                }
            }
            last = std::time::Instant::now();
            if let Ok(mut state) = status.lock() {
                *state = s;
            }
        }
    });
}
