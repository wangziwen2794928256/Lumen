use super::DatabaseState;
use crate::{database::Result, kernel::modules::Registry};
#[tauri::command]
pub async fn module_registry_get(state: tauri::State<'_, DatabaseState>) -> Result<Registry> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .module_registry()
        .await
}
#[tauri::command]
pub async fn module_registry_save(
    state: tauri::State<'_, DatabaseState>,
    value: Registry,
) -> Result<Registry> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .module_save(value)
        .await
}
