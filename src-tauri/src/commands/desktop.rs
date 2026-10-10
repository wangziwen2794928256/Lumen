use super::DatabaseState;
use crate::database::Result;
use crate::kernel::desktop::{self, OnboardingInput, Preferences, Status};
#[tauri::command]
pub async fn desktop_status(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Status> {
    desktop::status(&app, state.0.as_ref().map_err(Clone::clone)?).await
}
#[tauri::command]
pub async fn desktop_preferences_save(
    state: tauri::State<'_, DatabaseState>,
    preferences: Preferences,
) -> Result<Preferences> {
    desktop::save_preferences(state.0.as_ref().map_err(Clone::clone)?, preferences).await
}
#[tauri::command]
pub fn desktop_autostart(app: tauri::AppHandle, enabled: bool) -> Result<bool> {
    desktop::set_autostart(&app, enabled)
}
#[tauri::command]
pub async fn desktop_test_notification(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<()> {
    desktop::test_notification(&app, state.0.as_ref().map_err(Clone::clone)?).await
}
#[tauri::command]
pub async fn onboarding_complete(
    state: tauri::State<'_, DatabaseState>,
    input: OnboardingInput,
) -> Result<()> {
    desktop::complete_onboarding(state.0.as_ref().map_err(Clone::clone)?, input).await
}
#[tauri::command]
pub async fn onboarding_skip(state: tauri::State<'_, DatabaseState>) -> Result<()> {
    desktop::skip_onboarding(state.0.as_ref().map_err(Clone::clone)?).await
}
