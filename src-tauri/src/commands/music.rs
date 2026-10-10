use super::DatabaseState;
use crate::{
    database::{DbError, Result},
    music::{Action, MediaState, MediaStatus},
};
#[tauri::command]
pub async fn music_snapshot(
    state: tauri::State<'_, DatabaseState>,
    from: String,
    until: String,
    trash: bool,
) -> Result<serde_json::Value> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.music").await?;
    db.music_snapshot(from, until, trash).await
}
#[tauri::command]
pub async fn music_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.music").await?;
    db.music_action(action).await
}
#[tauri::command]
pub fn music_status(state: tauri::State<'_, MediaState>) -> Result<MediaStatus> {
    state
        .0
        .lock()
        .map(|s| s.clone())
        .map_err(|_| DbError::invalid())
}
