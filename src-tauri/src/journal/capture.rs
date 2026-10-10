use crate::database::{Database, DbError, Result};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
#[derive(Clone, Serialize)]
pub struct Status {
    pub shortcut: String,
    pub registered: bool,
    pub error: Option<String>,
}
pub struct CaptureState(pub Mutex<Status>);
pub fn open(app: &tauri::AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window("capture") {
        window.show().map_err(|_| DbError::invalid())?;
        window.set_focus().map_err(|_| DbError::invalid())?;
        return Ok(());
    }
    WebviewWindowBuilder::new(
        app,
        "capture",
        WebviewUrl::App("index.html#/capture".into()),
    )
    .title("Lumen · 快速记录")
    .inner_size(520., 560.)
    .min_inner_size(400., 420.)
    .always_on_top(true)
    .build()
    .map_err(|_| DbError::new("CAPTURE_UNAVAILABLE", "快速记录窗口无法打开。"))?;
    Ok(())
}
fn parse(value: &str) -> Result<Shortcut> {
    if value.len() > 80 || !value.contains('+') {
        return Err(DbError::invalid());
    }
    value.parse().map_err(|_| {
        DbError::new(
            "SHORTCUT_INVALID",
            "快捷键格式无效，请使用 Control+Shift+Space 等组合。",
        )
    })
}
pub fn register(app: &tauri::AppHandle, value: &str) -> Result<()> {
    let shortcut = parse(value)?;
    app.global_shortcut()
        .on_shortcut(shortcut, |app, _, event| {
            if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = open(&handle);
                });
            }
        })
        .map_err(|_| {
            DbError::new(
                "SHORTCUT_CONFLICT",
                "快捷键被占用，请修改组合；仍可用按钮打开快速记录。",
            )
        })
}
pub async fn initialize(app: &tauri::AppHandle, db: &Database) {
    let value: Option<String> =
        sqlx::query_scalar("SELECT value_text FROM settings WHERE key='journal_shortcut'")
            .fetch_optional(&db.pool)
            .await
            .unwrap_or(None);
    let shortcut = value.unwrap_or_else(|| "Control+Shift+Space".into());
    let result = register(app, &shortcut);
    app.manage(CaptureState(Mutex::new(Status {
        shortcut,
        registered: result.is_ok(),
        error: result.err().map(|e| e.message.into()),
    })));
}
pub async fn update(app: &tauri::AppHandle, db: &Database, value: String) -> Result<Status> {
    let old = app
        .state::<CaptureState>()
        .0
        .lock()
        .map_err(|_| DbError::invalid())?
        .clone();
    let new = parse(&value)?;
    if old.shortcut == value && old.registered {
        return Ok(old);
    }
    register(app, &value)?; // Reserve replacement first; preserve working old binding on conflict.
    let saved=sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'journal_shortcut',?,?,?) ON CONFLICT(key) DO UPDATE SET value_text=excluded.value_text,updated_at=excluded.updated_at").bind(uuid::Uuid::new_v4().to_string()).bind(&value).bind(crate::database::now()).bind(crate::database::now()).execute(&db.pool).await;
    if let Err(e) = saved {
        let _ = app.global_shortcut().unregister(new);
        return Err(e.into());
    }
    if old.registered {
        let _ = app.global_shortcut().unregister(parse(&old.shortcut)?);
    }
    let status = Status {
        shortcut: value,
        registered: true,
        error: None,
    };
    *app.state::<CaptureState>()
        .0
        .lock()
        .map_err(|_| DbError::invalid())? = status.clone();
    Ok(status)
}
