use super::DatabaseState;
use crate::database::{backup, backup_schedule, Result};
#[derive(serde::Serialize)]
pub struct BackupStatus {
    policy: backup_schedule::Policy,
    scheduler: backup_schedule::SchedulerStatus,
}
#[tauri::command]
pub async fn backup_status(state: tauri::State<'_, DatabaseState>) -> Result<BackupStatus> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    Ok(BackupStatus {
        policy: backup_schedule::policy(db).await?,
        scheduler: backup_schedule::status(),
    })
}
#[tauri::command]
pub async fn backup_policy_save(
    state: tauri::State<'_, DatabaseState>,
    policy: backup_schedule::Policy,
) -> Result<()> {
    backup_schedule::save_policy(state.0.as_ref().map_err(Clone::clone)?, policy).await
}
#[tauri::command]
pub async fn backup_create(state: tauri::State<'_, DatabaseState>) -> Result<String> {
    let path = backup::create_complete(state.0.as_ref().map_err(Clone::clone)?).await?;
    Ok(path
        .file_name()
        .ok_or_else(crate::database::DbError::backup)?
        .to_string_lossy()
        .into_owned())
}

pub struct ImportState(pub std::sync::Mutex<Option<(String, String)>>);
#[derive(serde::Serialize)]
pub struct ImportReview {
    pub token: String,
    pub review: crate::kernel::data_exchange::Review,
}
#[tauri::command]
pub async fn privacy_status(
    state: tauri::State<'_, DatabaseState>,
) -> Result<crate::kernel::privacy::PrivacyStatus> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .privacy_status()
        .await
}
#[tauri::command]
pub async fn privacy_disable(state: tauri::State<'_, DatabaseState>, delete: bool) -> Result<()> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .privacy_disable(delete)
        .await
}
#[tauri::command]
pub fn privacy_open_folder(state: tauri::State<'_, DatabaseState>) -> Result<()> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    crate::kernel::platform::open_data_directory(&db.root)
}
#[tauri::command]
pub async fn private_export(
    state: tauri::State<'_, DatabaseState>,
    format: String,
) -> Result<String> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .private_export(&format)
        .await
}
#[tauri::command]
pub async fn import_review(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    pending: tauri::State<'_, ImportState>,
) -> Result<Option<ImportReview>> {
    use tauri_plugin_dialog::DialogExt;
    let file = app
        .dialog()
        .file()
        .add_filter("Lumen JSON data backup", &["json"])
        .blocking_pick_file();
    let Some(file) = file else { return Ok(None) };
    let path = file
        .into_path()
        .map_err(|_| crate::database::DbError::invalid())?;
    if std::fs::metadata(&path)?.len() > 64 * 1024 * 1024 {
        return Err(crate::database::DbError::invalid());
    }
    let text = std::fs::read_to_string(path)?;
    if text.len() > 64 * 1024 * 1024 {
        return Err(crate::database::DbError::invalid());
    }
    let review = state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .exchange_import(&text, None)
        .await?;
    let token = uuid::Uuid::new_v4().to_string();
    *pending
        .0
        .lock()
        .map_err(|_| crate::database::DbError::invalid())? = Some((token.clone(), text));
    Ok(Some(ImportReview { token, review }))
}
#[tauri::command]
pub async fn import_confirm(
    state: tauri::State<'_, DatabaseState>,
    pending: tauri::State<'_, ImportState>,
    token: String,
    fingerprint: String,
) -> Result<crate::kernel::data_exchange::Review> {
    let text = {
        let p = pending
            .0
            .lock()
            .map_err(|_| crate::database::DbError::invalid())?;
        p.as_ref()
            .filter(|(t, _)| t == &token)
            .map(|(_, s)| s.clone())
            .ok_or_else(crate::database::DbError::invalid)?
    };
    let db = state.0.as_ref().map_err(Clone::clone)?;
    // Complete pre-import snapshot; any failure stops import before live writes.
    backup::create_complete(db).await?;
    let review = db.exchange_import(&text, Some(&fingerprint)).await?;
    let mut p = pending
        .0
        .lock()
        .map_err(|_| crate::database::DbError::invalid())?;
    if p.as_ref().is_some_and(|(t, _)| t == &token) {
        *p = None;
    }
    Ok(review)
}
#[tauri::command]
pub async fn restore_rehearse(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Option<crate::kernel::privacy::RestoreReview>> {
    use tauri_plugin_dialog::DialogExt;
    let folder = app.dialog().file().blocking_pick_folder();
    let Some(folder) = folder else {
        return Ok(None);
    };
    let path = folder
        .into_path()
        .map_err(|_| crate::database::DbError::invalid())?;
    let db = state.0.as_ref().map_err(Clone::clone)?.clone();
    let review = tauri::async_runtime::spawn_blocking(move || {
        tauri::async_runtime::block_on(crate::kernel::privacy::rehearse(&db, &path))
    })
    .await
    .map_err(|_| crate::database::DbError::backup())??;
    Ok(Some(review))
}
