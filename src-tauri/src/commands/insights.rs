use super::DatabaseState;
use crate::{
    database::Result,
    insights::{Action, Filter, Record, Snapshot},
};
#[tauri::command]
pub async fn insights_snapshot(
    state: tauri::State<'_, DatabaseState>,
    filter: Filter,
) -> Result<Snapshot> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .require_module("lumen.insights")
        .await?;
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .insights_snapshot(filter)
        .await
}
#[tauri::command]
pub async fn insights_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .require_module("lumen.insights")
        .await?;
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .insights_action(action)
        .await
}
#[tauri::command]
pub async fn insights_review(
    state: tauri::State<'_, DatabaseState>,
    week: String,
) -> Result<serde_json::Value> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .require_module("lumen.insights")
        .await?;
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .insights_review(week)
        .await
}
#[tauri::command]
pub async fn insights_month(
    state: tauri::State<'_, DatabaseState>,
    month: String,
    today: String,
) -> Result<serde_json::Value> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .require_module("lumen.insights")
        .await?;
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .insights_month(month, today)
        .await
}
#[tauri::command]
pub async fn insights_memories(
    state: tauri::State<'_, DatabaseState>,
    today: String,
) -> Result<Vec<Record>> {
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .require_module("lumen.insights")
        .await?;
    state
        .0
        .as_ref()
        .map_err(Clone::clone)?
        .insights_memories(today)
        .await
}
