use super::DatabaseState;
use crate::{
    database::{DbError, Result},
    vault::Action,
};
#[tauri::command]
pub async fn vault_snapshot(state: tauri::State<'_, DatabaseState>) -> Result<serde_json::Value> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.vault").await?;
    db.vault_snapshot().await
}
#[tauri::command]
pub async fn vault_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.vault").await?;
    db.vault_action(action).await
}
#[tauri::command]
pub async fn vault_files(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<serde_json::Value> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.vault").await?;
    db.vault_files(&id).await
}
#[tauri::command]
pub async fn vault_pick_attachment(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    id: String,
    revision: i64,
) -> Result<()> {
    use tauri_plugin_dialog::DialogExt;
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.vault").await?;
    let file = app
        .dialog()
        .file()
        .add_filter("图片或 PDF", &["pdf", "png", "jpg", "jpeg", "webp"])
        .blocking_pick_file();
    if let Some(file) = file {
        db.vault_import(
            &id,
            revision,
            &file.into_path().map_err(|_| DbError::invalid())?,
        )
        .await?;
    }
    Ok(())
}
#[tauri::command]
pub async fn vault_verify(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    open: bool,
) -> Result<String> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.vault").await?;
    let (status, path) = db.vault_verify(&id).await?;
    if open {
        let path = path
            .ok_or_else(|| DbError::new("ATTACHMENT_INVALID", "附件缺失或哈希不匹配，未打开。"))?;
        crate::kernel::platform::open_verified_attachment(&path)?;
    }
    Ok(status)
}
