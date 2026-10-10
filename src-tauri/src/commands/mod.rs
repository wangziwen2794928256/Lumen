mod desktop;
mod privacy;
pub use desktop::*;
mod profile;
pub use privacy::*;
mod vault;
pub use profile::*;
pub use vault::*;
mod modules;
mod music;
pub use modules::*;
pub use music::*;
mod insights;
pub use insights::*;
mod coding;
pub use coding::*;
mod journal;
pub use journal::*;
#[derive(serde::Serialize)]
// Journal commands keep native file selection separate from renderer-supplied input.
pub struct AppInfo {
    name: &'static str,
    version: &'static str,
    platform: &'static str,
}
pub struct DatabaseState(pub crate::database::Result<crate::database::Database>);
pub struct TodayState(pub crate::database::Result<std::sync::Arc<crate::today::TodayService>>);
#[tauri::command]
pub async fn roadmap_snapshot(
    state: tauri::State<'_, TodayState>,
    week: String,
) -> crate::database::Result<crate::today::roadmap::Snapshot> {
    match &state.0 {
        Ok(s) => s.roadmap_snapshot(&week).await,
        Err(e) => Err(e.clone()),
    }
}
#[tauri::command]
pub async fn roadmap_action(
    state: tauri::State<'_, TodayState>,
    week: String,
    action: crate::today::roadmap::Action,
) -> crate::database::Result<crate::today::roadmap::Snapshot> {
    match &state.0 {
        Ok(s) => s.roadmap_action(&week, action).await,
        Err(e) => Err(e.clone()),
    }
}
#[tauri::command]
pub async fn calendar_snapshot(
    state: tauri::State<'_, TodayState>,
    from: String,
    until: String,
) -> crate::database::Result<crate::today::calendar::CalendarSnapshot> {
    match &state.0 {
        Ok(service) => service.calendar_snapshot(&from, &until).await,
        Err(error) => Err(error.clone()),
    }
}
#[tauri::command]
pub async fn today_snapshot(
    state: tauri::State<'_, TodayState>,
) -> crate::database::Result<crate::today::Snapshot> {
    match &state.0 {
        Ok(service) => service.snapshot().await,
        Err(error) => Err(error.clone()),
    }
}
#[tauri::command]
pub async fn today_action(
    state: tauri::State<'_, TodayState>,
    action: crate::today::Action,
) -> crate::database::Result<crate::today::Snapshot> {
    match &state.0 {
        Ok(service) => service.act(action).await,
        Err(error) => Err(error.clone()),
    }
}
#[tauri::command]
pub async fn database_status(
    state: tauri::State<'_, DatabaseState>,
) -> crate::database::Result<crate::database::DatabaseStatus> {
    match &state.0 {
        Ok(db) => db.status().await,
        Err(error) => Err(error.clone()),
    }
}
#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Lumen",
        version: env!("CARGO_PKG_VERSION"),
        platform: crate::kernel::platform::host().name,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_info_matches_package_and_platform() {
        let info = app_info();
        assert_eq!(info.name, "Lumen");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.platform, crate::kernel::platform::host().name);
    }
}
