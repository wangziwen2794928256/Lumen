use super::DatabaseState;
use crate::{database::Result, journal::*};
#[tauri::command]
pub async fn journal_page(state: tauri::State<'_, DatabaseState>, filter: Filter) -> Result<Page> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_page(filter)
        .await
}
#[tauri::command]
pub async fn journal_get(state: tauri::State<'_, DatabaseState>, id: String) -> Result<Entry> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_get(&id)
        .await
}
#[tauri::command]
pub async fn journal_save(state: tauri::State<'_, DatabaseState>, input: Save) -> Result<Entry> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_save(input)
        .await
}
#[tauri::command]
pub async fn journal_trash(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    revision: i64,
    restore: bool,
) -> Result<Entry> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_trash(&id, revision, restore)
        .await
}
#[tauri::command]
pub async fn journal_attachments(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<Vec<Attachment>> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_attachments(&id)
        .await
}
#[tauri::command]
pub async fn journal_pick_attachment(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<Vec<Attachment>> {
    use tauri_plugin_dialog::DialogExt;
    let db = state.0.as_ref().map_err(Clone::clone)?;
    let file = app
        .dialog()
        .file()
        .add_filter("图片或 PDF", &["png", "jpg", "jpeg", "webp", "pdf"])
        .blocking_pick_file();
    if let Some(file) = file {
        let path = file
            .into_path()
            .map_err(|_| crate::database::DbError::invalid())?;
        db.journal_import(&id, &path).await?;
    }
    db.journal_attachments(&id).await
}
#[tauri::command]
pub async fn journal_link(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    url: String,
) -> Result<Vec<Attachment>> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.journal_link(&id, &url).await?;
    db.journal_attachments(&id).await
}
#[tauri::command]
pub async fn journal_verify(state: tauri::State<'_, DatabaseState>, id: String) -> Result<String> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_verify(&id)
        .await
}
#[tauri::command]
pub async fn capture_open(app: tauri::AppHandle) -> Result<()> {
    capture::open(&app)
}
#[tauri::command]
pub async fn journal_open_attachment(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<()> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .journal_open_attachment(&id)
        .await
}
#[tauri::command]
pub fn capture_close(app: tauri::AppHandle) -> Result<()> {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("capture") {
        window
            .close()
            .map_err(|_| crate::database::DbError::invalid())?;
    }
    Ok(())
}
#[tauri::command]
pub fn capture_status(state: tauri::State<'_, capture::CaptureState>) -> Result<capture::Status> {
    Ok(state
        .0
        .lock()
        .map_err(|_| crate::database::DbError::invalid())?
        .clone())
}
#[tauri::command]
pub async fn capture_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    shortcut: String,
) -> Result<capture::Status> {
    capture::update(&app, state.0.as_ref().map_err(Clone::clone)?, shortcut).await
}
