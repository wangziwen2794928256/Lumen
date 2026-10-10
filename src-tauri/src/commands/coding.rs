use super::DatabaseState;
use crate::{
    coding::{Action, Filter, Snapshot},
    database::Result,
};
#[tauri::command]
pub async fn coding_snapshot(
    state: tauri::State<'_, DatabaseState>,
    filter: Filter,
) -> Result<Snapshot> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .coding_snapshot(filter)
        .await
}
#[tauri::command]
pub async fn coding_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .coding_action(action)
        .await
}
