use super::DatabaseState;
use crate::{
    database::{DbError, Result},
    kernel::public_export::{ExportReceipt, Profile},
};
#[tauri::command]
pub async fn profile_get(state: tauri::State<'_, DatabaseState>) -> Result<Profile> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.profile").await?;
    db.profile_get().await
}
#[tauri::command]
pub async fn profile_save(state: tauri::State<'_, DatabaseState>, profile: Profile) -> Result<()> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.profile").await?;
    db.profile_save(profile).await
}
#[tauri::command]
pub async fn public_projection(
    state: tauri::State<'_, DatabaseState>,
) -> Result<serde_json::Value> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.profile").await?;
    let projection = db.public_projection().await?;
    let review_hash = projection.review_hash()?;
    let mut value = serde_json::to_value(projection).map_err(|_| DbError::invalid())?;
    value["review_hash"] = review_hash.into();
    Ok(value)
}
#[tauri::command]
pub async fn public_export(
    state: tauri::State<'_, DatabaseState>,
    expected_hash: String,
) -> Result<ExportReceipt> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.profile").await?;
    db.public_export(&expected_hash).await
}
#[tauri::command]
pub async fn public_preview(
    state: tauri::State<'_, DatabaseState>,
    expected_hash: String,
) -> Result<String> {
    let db = state.0.as_ref().map_err(Clone::clone)?;
    db.require_module("lumen.profile").await?;
    let projection = db.public_projection().await?;
    projection.require_review(&expected_hash)?;
    let json = serde_json::to_vec(&projection).map_err(|_| DbError::invalid())?;
    let url = crate::kernel::preview::start(json)?;
    crate::kernel::platform::open_local_preview(&url)?;
    Ok(url)
}
#[tauri::command]
pub fn deep_link_initial() -> Option<String> {
    std::env::args()
        .skip(1)
        .find_map(|s| crate::kernel::deep_link::route(&s).ok())
}
#[tauri::command]
pub fn deep_link_register() -> Result<()> {
    crate::kernel::platform::register_deep_link()
}
