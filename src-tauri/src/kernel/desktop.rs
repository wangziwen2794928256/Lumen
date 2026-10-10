use crate::database::{now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Preferences {
    pub close_to_tray: bool,
    pub notifications_enabled: bool,
    pub focus_finished: bool,
    pub upcoming_event: bool,
    pub weekly_review: bool,
    pub backup_failure: bool,
    pub revision: i64,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            close_to_tray: false,
            notifications_enabled: false,
            focus_finished: true,
            upcoming_event: true,
            weekly_review: true,
            backup_failure: true,
            revision: 0,
        }
    }
}
#[derive(Clone, Serialize, Default)]
pub struct RuntimeStatus {
    pub tray_ready: bool,
    pub error_code: Option<&'static str>,
}
pub struct DesktopState(pub Mutex<RuntimeStatus>);
#[derive(Serialize)]
pub struct Status {
    pub preferences: Preferences,
    pub autostart: Option<bool>,
    pub notification_permission: String,
    pub runtime: RuntimeStatus,
    pub onboarding: bool,
}
pub async fn preferences(db: &Database) -> Result<Preferences> {
    let text: Option<String> = sqlx::query_scalar(
        "SELECT value_text FROM settings WHERE key='desktop.v1' AND deleted_at IS NULL",
    )
    .fetch_optional(&db.pool)
    .await?;
    text.map(|s| serde_json::from_str(&s).map_err(|_| DbError::invalid()))
        .unwrap_or_else(|| Ok(Preferences::default()))
}
pub async fn save_preferences(db: &Database, input: Preferences) -> Result<Preferences> {
    let mut tx = db.pool.begin_with("BEGIN IMMEDIATE").await?;
    let text: Option<String> = sqlx::query_scalar(
        "SELECT value_text FROM settings WHERE key='desktop.v1' AND deleted_at IS NULL",
    )
    .fetch_optional(&mut *tx)
    .await?;
    let current: Preferences = text
        .map(|s| serde_json::from_str(&s).map_err(|_| DbError::invalid()))
        .transpose()?
        .unwrap_or_default();
    if current.revision != input.revision {
        return Err(DbError::new("STATE_CONFLICT", "设置已变化，请重新载入。"));
    }
    let saved = Preferences {
        revision: current
            .revision
            .checked_add(1)
            .ok_or_else(DbError::invalid)?,
        ..input
    };
    let value = serde_json::to_string(&saved).map_err(|_| DbError::invalid())?;
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'desktop.v1',?,?,?) ON CONFLICT(key) DO UPDATE SET value_text=excluded.value_text,updated_at=excluded.updated_at,deleted_at=NULL").bind(uuid::Uuid::new_v4().to_string()).bind(value).bind(now()).bind(now()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(saved)
}
pub fn record_error(app: &tauri::AppHandle, code: &'static str) {
    if let Some(s) = app.try_state::<DesktopState>() {
        if let Ok(mut s) = s.0.lock() {
            s.error_code = Some(code);
        }
    }
}
pub fn show(app: &tauri::AppHandle, route: Option<&str>) {
    if let Some(w) = app.get_webview_window("main") {
        // Always attempt show even if unminimize/focus is unavailable on the current desktop.
        let shown = w.show();
        let restored = w.unminimize();
        let focused = w.set_focus();
        if shown.is_err() || restored.is_err() || focused.is_err() {
            record_error(app, "WINDOW_UNAVAILABLE");
        }
        if let Some(route) = route {
            let _ = w.emit("lumen:navigate", route);
        }
    } else {
        record_error(app, "WINDOW_MISSING");
    }
}
pub fn second_instance(app: &tauri::AppHandle, args: Vec<String>) {
    let route = args
        .iter()
        .skip(1)
        .find_map(|s| crate::kernel::deep_link::route(s).ok());
    // Windows delivers this callback inside WM_COPYDATA. Return before dispatching
    // window operations so a hidden window cannot block the sender/event loop.
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        show(&app, route.as_deref());
    });
}
pub async fn status(app: &tauri::AppHandle, db: &Database) -> Result<Status> {
    let runtime = app
        .state::<DesktopState>()
        .0
        .lock()
        .map_err(|_| DbError::invalid())?
        .clone();
    let onboarding = needs_onboarding(db).await?;
    Ok(Status {
        preferences: preferences(db).await?,
        autostart: app.autolaunch().is_enabled().ok(),
        notification_permission: format!(
            "{:?}",
            app.notification()
                .permission_state()
                .map_err(|_| DbError::new("NOTIFICATION_UNAVAILABLE", "系统通知状态不可用。"))?
        ),
        runtime,
        onboarding,
    })
}
pub fn set_autostart(app: &tauri::AppHandle, enabled: bool) -> Result<bool> {
    let manager = app.autolaunch();
    if enabled {
        manager.enable()
    } else {
        manager.disable()
    }
    .map_err(|_| {
        DbError::new(
            "AUTOSTART_UNAVAILABLE",
            "开机启动未更新，请检查系统限制；应用仍可手动打开。",
        )
    })?;
    manager.is_enabled().map_err(|_| DbError::invalid())
}
pub fn initialize(app: &tauri::AppHandle, db: Option<Database>) {
    app.manage(DesktopState(Mutex::new(RuntimeStatus::default())));
    match install_tray(app) {
        Ok(()) => {
            if let Ok(mut s) = app.state::<DesktopState>().0.lock() {
                s.tray_ready = true;
            }
        }
        Err(_) => record_error(app, "TRAY_UNAVAILABLE"),
    }
    if let Some(db) = db {
        if let Some(window) = app.get_webview_window("main") {
            let app = app.clone();
            let db = db.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let available = app
                        .state::<DesktopState>()
                        .0
                        .lock()
                        .map(|s| s.tray_ready)
                        .unwrap_or(false);
                    if available
                        && tauri::async_runtime::block_on(preferences(&db))
                            .map(|p| p.close_to_tray)
                            .unwrap_or(false)
                    {
                        let _ = app.save_window_state(
                            StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED,
                        );
                        if let Some(w) = app.get_webview_window("main") {
                            // Cancel the close before requesting any further window operations.
                            api.prevent_close();
                            if w.hide().is_err() {
                                record_error(&app, "WINDOW_UNAVAILABLE");
                                show(&app, None);
                            }
                        }
                    }
                }
            });
        }
        spawn_notifications(app.clone(), db);
    }
}
fn install_tray(app: &tauri::AppHandle) -> std::result::Result<(), tauri::Error> {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
    let open = MenuItem::with_id(app, "open", "打开 Lumen", true, None::<&str>)?;
    let start = MenuItem::with_id(app, "focus", "开始专注", true, None::<&str>)?;
    let capture = MenuItem::with_id(app, "capture", "快速记录", true, None::<&str>)?;
    let pause = MenuItem::with_id(app, "pause", "暂停专注", true, None::<&str>)?;
    let today = MenuItem::with_id(app, "today", "今日", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出 Lumen", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &start, &capture, &pause, &today, &quit])?;
    // Deterministic RGBA icon; no external asset, executable or network download.
    let mut pixels = vec![0u8; 32 * 32 * 4];
    for y in 0..32 {
        for x in 0..32 {
            if (3..29).contains(&x) && (3..29).contains(&y) {
                let i = (y * 32 + x) * 4;
                pixels[i..i + 4].copy_from_slice(&[69, 95, 77, 255]);
                if (9..13).contains(&x) && (8..24).contains(&y)
                    || (9..24).contains(&x) && (20..24).contains(&y)
                {
                    pixels[i..i + 4].copy_from_slice(&[244, 243, 237, 255]);
                }
            }
        }
    }
    TrayIconBuilder::with_id("lumen-tray")
        .icon(tauri::image::Image::new_owned(pixels, 32, 32))
        .tooltip("Lumen")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                show(tray.app_handle(), None);
            }
        })
        .on_menu_event(|app, event| {
            let app = app.clone();
            let id = event.id().as_ref().to_string();
            tauri::async_runtime::spawn(async move {
                if tray_action(&app, &id).await.is_err() {
                    record_error(&app, "TRAY_ACTION_UNAVAILABLE");
                    show(&app, None);
                }
            });
        })
        .build(app)?;
    Ok(())
}
async fn tray_action(app: &tauri::AppHandle, id: &str) -> Result<()> {
    match id {
        "open" => show(app, None),
        "today" => show(app, Some("/")),
        "capture" => crate::journal::capture::open(app)?,
        "focus" | "pause" => {
            let state = app.state::<crate::commands::TodayState>();
            let service = state.0.as_ref().map_err(Clone::clone)?;
            let snapshot = service.snapshot().await?;
            if id == "pause" {
                if let Some(s) = snapshot.session.filter(|s| s.state == "running") {
                    service
                        .act(crate::today::Action::Session {
                            id: s.id,
                            operation: "pause".into(),
                        })
                        .await?;
                }
            } else if snapshot.session.is_none() {
                if let Some(task) = snapshot
                    .tasks
                    .iter()
                    .find(|t| matches!(t.lifecycle.as_str(), "active" | "planned"))
                {
                    service
                        .act(crate::today::Action::Start {
                            id: uuid::Uuid::new_v4().to_string(),
                            task_id: task.id.clone(),
                        })
                        .await?;
                }
            }
            show(app, Some("/focus"));
        }
        "quit" => {
            let _ = app
                .save_window_state(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED);
            app.exit(0);
        }
        _ => return Err(DbError::invalid()),
    }
    Ok(())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Notice {
    Focus,
    Event,
    Review,
    Backup,
}
impl Notice {
    fn allowed(self, p: &Preferences) -> bool {
        p.notifications_enabled
            && match self {
                Self::Focus => p.focus_finished,
                Self::Event => p.upcoming_event,
                Self::Review => p.weekly_review,
                Self::Backup => p.backup_failure,
            }
    }
    fn body(self) -> &'static str {
        match self {
            Self::Focus => "本次专注已结束。",
            Self::Event => "即将开始一项固定日程，可在日历查看。",
            Self::Review => "上周记录可以回顾了。",
            Self::Backup => "本地备份未完成，请在设置中检查。",
        }
    }
}
fn send(app: &tauri::AppHandle, notice: Notice) -> Result<()> {
    // No permission prompts on startup. Desktop plugin reports API availability, not OS delivery guarantees.
    if !matches!(
        app.notification().permission_state(),
        Ok(tauri_plugin_notification::PermissionState::Granted)
    ) {
        return Err(DbError::new(
            "NOTIFICATION_DENIED",
            "系统通知不可用，可在应用内查看状态。",
        ));
    }
    app.notification()
        .builder()
        .title("Lumen")
        .body(notice.body())
        .show()
        .map_err(|_| {
            DbError::new(
                "NOTIFICATION_UNAVAILABLE",
                "通知未发送，应用内功能不受影响。",
            )
        })
}
pub async fn test_notification(app: &tauri::AppHandle, db: &Database) -> Result<()> {
    if !Notice::Focus.allowed(&preferences(db).await?) {
        return Err(DbError::invalid());
    }
    send(app, Notice::Focus)
}
fn spawn_notifications(app: tauri::AppHandle, db: Database) {
    tauri::async_runtime::spawn(async move {
        let mut since = now();
        let mut after_id = String::new();
        let mut seen = std::collections::HashMap::new();
        let mut backup_error = None;
        let mut weekly = String::new();
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let Ok(p) = preferences(&db).await else {
                continue;
            };
            let mut notices = Vec::new();
            if let Ok(rows)=sqlx::query_as::<_,(String,String)>("SELECT id,updated_at FROM focus_sessions WHERE state='finished' AND deleted_at IS NULL AND (updated_at>? OR (updated_at=? AND id>?)) ORDER BY updated_at,id LIMIT 100").bind(&since).bind(&since).bind(&after_id).fetch_all(&db.pool).await {for (id,stamp) in rows{since=stamp;after_id=id;notices.push(Notice::Focus);}}
            let current = chrono::Local::now();
            let start = current.to_rfc3339();
            let end = (current + chrono::Duration::minutes(5)).to_rfc3339();
            seen.retain(|_, expiry: &mut chrono::DateTime<chrono::Local>| *expiry > current);
            if let Ok(ids) = upcoming_events(&db, &start, &end).await {
                for id in ids {
                    if let std::collections::hash_map::Entry::Vacant(entry) = seen.entry(id) {
                        entry.insert(current + chrono::Duration::minutes(6));
                        notices.push(Notice::Event);
                    }
                }
            }
            let week = current.format("%G-W%V").to_string();
            if week != weekly {
                weekly = week;
                use chrono::Datelike;
                let monday = current.date_naive()
                    - chrono::Duration::days(current.weekday().num_days_from_monday().into());
                let previous = monday - chrono::Duration::days(7);
                if sqlx::query_scalar::<_,i64>("SELECT count(*) FROM journal_entries WHERE deleted_at IS NULL AND date(created_at,'localtime')>=? AND date(created_at,'localtime')<?").bind(previous.to_string()).bind(monday.to_string()).fetch_one(&db.pool).await.unwrap_or(0)>0{notices.push(Notice::Review);}
            }
            let error = crate::database::backup_schedule::status().error_code;
            if error != backup_error {
                if error.is_some() {
                    notices.push(Notice::Backup);
                }
                backup_error = error;
            }
            for notice in notices {
                if notice.allowed(&p) && send(&app, notice).is_err() {
                    record_error(&app, "NOTIFICATION_UNAVAILABLE");
                }
            }
        }
    });
}

async fn upcoming_events(db: &Database, start: &str, end: &str) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar("SELECT id || ':' || starts_at FROM calendar_events WHERE deleted_at IS NULL AND archived_at IS NULL AND starts_at IS NOT NULL AND julianday(starts_at)>=julianday(?) AND julianday(starts_at)<=julianday(?) ORDER BY starts_at,id LIMIT 100").bind(start).bind(end).fetch_all(&db.pool).await?)
}

pub async fn needs_onboarding(db: &Database) -> Result<bool> {
    let completed: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM settings WHERE key='onboarding.v1' AND deleted_at IS NULL",
    )
    .fetch_one(&db.pool)
    .await?;
    let existing: i64 = sqlx::query_scalar(EXISTING_RECORDS)
        .fetch_one(&db.pool)
        .await?;
    Ok(completed == 0 && existing == 0)
}
const EXISTING_RECORDS: &str = "SELECT (SELECT count(*) FROM tasks)+(SELECT count(*) FROM journeys)+(SELECT count(*) FROM subjects)+(SELECT count(*) FROM journal_entries)+(SELECT count(*) FROM music_plays)+(SELECT count(*) FROM artifacts)+(SELECT count(*) FROM calendar_events)+(SELECT count(*) FROM coding_attempts)";
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnboardingInput {
    pub journey: String,
    pub subjects: Vec<String>,
    pub weekday_minutes: i64,
    pub weekend_minutes: i64,
}
pub async fn complete_onboarding(db: &Database, input: OnboardingInput) -> Result<()> {
    use crate::today::roadmap::model::{Action, Details, EntityInput, Preferences as Planning};
    if input.journey.chars().count() > 250
        || input.subjects.len() > 12
        || input
            .subjects
            .iter()
            .any(|s| s.trim().is_empty() || s.chars().count() > 250)
        || !(0..=1440).contains(&input.weekday_minutes)
        || !(0..=1440).contains(&input.weekend_minutes)
    {
        return Err(DbError::invalid());
    }
    let mut tx = db.pool.begin_with("BEGIN IMMEDIATE").await?;
    let completed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM settings WHERE key='onboarding.v1'")
            .fetch_one(&mut *tx)
            .await?;
    if completed > 0 {
        return Ok(());
    }
    let existing: i64 = sqlx::query_scalar(EXISTING_RECORDS)
        .fetch_one(&mut *tx)
        .await?;
    if existing > 0 {
        return Err(DbError::new(
            "STATE_CONFLICT",
            "已有记录，请在路线图和设置中继续配置。",
        ));
    }
    let journey_id = (!input.journey.trim().is_empty()).then(|| uuid::Uuid::new_v4().to_string());
    if let Some(id) = &journey_id {
        crate::today::roadmap::mutations::write(
            &mut tx,
            &Action::Save {
                id: id.clone(),
                kind: "journey".into(),
                revision: None,
                input: EntityInput {
                    title: input.journey.trim().into(),
                    journey_id: None,
                    data: Details::default(),
                },
            },
            &now(),
        )
        .await?;
    }
    let mut seen = std::collections::BTreeSet::new();
    for title in &input.subjects {
        let title = title.trim();
        if seen.insert(title) {
            crate::today::roadmap::mutations::write(
                &mut tx,
                &Action::Save {
                    id: uuid::Uuid::new_v4().to_string(),
                    kind: "subject".into(),
                    revision: None,
                    input: EntityInput {
                        title: title.into(),
                        journey_id: journey_id.clone(),
                        data: Details::default(),
                    },
                },
                &now(),
            )
            .await?;
        }
    }
    let revision = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE((SELECT revision FROM planning_preferences WHERE id=1),0)",
    )
    .fetch_one(&mut *tx)
    .await?;
    crate::today::roadmap::mutations::write(
        &mut tx,
        &Action::Preferences {
            input: Planning {
                weekday_minutes: input.weekday_minutes,
                weekend_minutes: input.weekend_minutes,
                revision,
                ..Planning::default()
            },
        },
        &now(),
    )
    .await?;
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'onboarding.v1','complete',?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(now()).bind(now()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
pub async fn skip_onboarding(db: &Database) -> Result<()> {
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'onboarding.v1','skipped',?,?) ON CONFLICT(key) DO NOTHING").bind(uuid::Uuid::new_v4().to_string()).bind(now()).bind(now()).execute(&db.pool).await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn calendar_notice_query_filters_deleted_archived_and_all_day() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        for (id, starts, date, archived, deleted) in [
            (
                "00000000-0000-4000-8000-000000000001",
                Some("2026-10-10T08:03:00Z"),
                None,
                None,
                None,
            ),
            (
                "00000000-0000-4000-8000-000000000002",
                Some("2026-10-10T08:06:00Z"),
                None,
                None,
                None,
            ),
            (
                "00000000-0000-4000-8000-000000000003",
                None,
                Some("2026-10-10"),
                None,
                None,
            ),
            (
                "00000000-0000-4000-8000-000000000004",
                Some("2026-10-10T08:04:00Z"),
                None,
                Some(now()),
                None,
            ),
            (
                "00000000-0000-4000-8000-000000000005",
                Some("2026-10-10T08:04:00Z"),
                None,
                None,
                Some(now()),
            ),
        ] {
            sqlx::query("INSERT INTO calendar_events(id,title,starts_at,event_date,archived_at,deleted_at,created_at,updated_at) VALUES(?,'isolated fixture',?,?,?,?,?,?)")
                .bind(id).bind(starts).bind(date).bind(archived).bind(deleted).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
        }
        let ids = upcoming_events(
            &db,
            "2026-10-10T16:00:00+08:00",
            "2026-10-10T16:05:00+08:00",
        )
        .await
        .unwrap();
        assert_eq!(
            ids,
            vec!["00000000-0000-4000-8000-000000000001:2026-10-10T08:03:00Z"]
        );
        assert!(!needs_onboarding(&db).await.unwrap());
        assert!(complete_onboarding(
            &db,
            OnboardingInput {
                journey: "Do not create".into(),
                subjects: vec![],
                weekday_minutes: 60,
                weekend_minutes: 60
            }
        )
        .await
        .is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM journeys")
                .fetch_one(&db.pool)
                .await
                .unwrap(),
            0
        );
    }
    #[tokio::test]
    async fn skipping_onboarding_creates_no_personal_records() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        skip_onboarding(&db).await.unwrap();
        skip_onboarding(&db).await.unwrap();
        assert!(!needs_onboarding(&db).await.unwrap());
        assert_eq!(
            sqlx::query_scalar::<_, i64>(EXISTING_RECORDS)
                .fetch_one(&db.pool)
                .await
                .unwrap(),
            0
        );
    }
    #[tokio::test]
    async fn onboarding_is_atomic_manual_and_idempotent() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        assert!(needs_onboarding(&db).await.unwrap());
        let input = OnboardingInput {
            journey: "学习".repeat(100),
            subjects: vec!["Subject A".into(), "Subject A".into()],
            weekday_minutes: 120,
            weekend_minutes: 60,
        };
        let mut bad = input.clone();
        bad.weekday_minutes = 1441;
        assert!(complete_onboarding(&db, bad).await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM journeys")
                .fetch_one(&db.pool)
                .await
                .unwrap(),
            0
        );
        complete_onboarding(&db, input.clone()).await.unwrap();
        complete_onboarding(&db, input).await.unwrap();
        assert!(!needs_onboarding(&db).await.unwrap());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM subjects")
                .fetch_one(&db.pool)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM tasks")
                .fetch_one(&db.pool)
                .await
                .unwrap(),
            0
        );
        let p = preferences(&db).await.unwrap();
        assert!(!p.notifications_enabled && !p.close_to_tray);
        let saved = save_preferences(
            &db,
            Preferences {
                close_to_tray: true,
                ..p.clone()
            },
        )
        .await
        .unwrap();
        assert_eq!(saved.revision, 1);
        assert!(save_preferences(&db, p).await.is_err());
    }
    #[test]
    fn notices_are_opt_in_and_allowlisted() {
        let mut p = Preferences::default();
        for n in [Notice::Focus, Notice::Event, Notice::Review, Notice::Backup] {
            assert!(!n.allowed(&p));
        }
        p.notifications_enabled = true;
        assert!(Notice::Focus.allowed(&p));
        p.focus_finished = false;
        assert!(!Notice::Focus.allowed(&p));
        assert!(Notice::Backup.allowed(&p));
        assert!(!Notice::Backup.body().contains("secret"));
    }
}
