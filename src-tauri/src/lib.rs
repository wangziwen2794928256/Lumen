pub mod coding;
mod commands;
pub mod database;
pub mod insights;
pub mod journal;
pub mod kernel;
pub mod modules;
pub mod music;
pub mod today;
pub mod vault;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            kernel::desktop::second_instance(app, args);
        }))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::SIZE
                        | tauri_plugin_window_state::StateFlags::POSITION
                        | tauri_plugin_window_state::StateFlags::MAXIMIZED,
                )
                .with_denylist(&["capture"])
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let root = app.path().app_data_dir()?;
            let state = tauri::async_runtime::block_on(database::Database::open(&root));
            let mut today_state = match &state {
                Ok(db) => tauri::async_runtime::block_on(today::TodayService::open(db.clone()))
                    .map(std::sync::Arc::new),
                Err(error) => Err(error.clone()),
            };
            if let (Ok(service), Some(window)) = (&today_state, app.get_webview_window("main")) {
                if let Err(error) = today::native_events::install(&window, service.clone()) {
                    today_state = Err(error);
                }
            }
            if let Ok(service) = &today_state {
                today::TodayService::spawn_checkpoint(service.clone());
            }
            if let Ok(db) = &state {
                database::backup_schedule::initialize(db.clone());
                tauri::async_runtime::block_on(journal::capture::initialize(app.handle(), db));
                modules::official::insights::initialize(db.clone());
                modules::official::music::initialize(app.handle(), db.clone());
            } else {
                app.manage(music::MediaState(std::sync::Arc::new(
                    std::sync::Mutex::new(music::MediaStatus {
                        state: "manual-fallback".into(),
                        error: Some("数据库不可用".into()),
                        ..Default::default()
                    }),
                )));
                app.manage(journal::capture::CaptureState(std::sync::Mutex::new(
                    journal::capture::Status {
                        shortcut: "Control+Shift+Space".into(),
                        registered: false,
                        error: Some("数据库不可用".into()),
                    },
                )));
            }
            app.manage(commands::TodayState(today_state));
            // Keep the app accessible even when storage cannot open; never recreate/reset corrupt data.
            app.manage(commands::DatabaseState(state));
            app.manage(commands::ImportState(std::sync::Mutex::new(None)));
            kernel::desktop::initialize(
                app.handle(),
                app.state::<commands::DatabaseState>()
                    .0
                    .as_ref()
                    .ok()
                    .cloned(),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::desktop_status,
            commands::desktop_preferences_save,
            commands::desktop_autostart,
            commands::desktop_test_notification,
            commands::onboarding_complete,
            commands::onboarding_skip,
            commands::privacy_status,
            commands::privacy_disable,
            commands::privacy_open_folder,
            commands::private_export,
            commands::import_review,
            commands::import_confirm,
            commands::restore_rehearse,
            commands::backup_status,
            commands::backup_policy_save,
            commands::backup_create,
            commands::profile_get,
            commands::profile_save,
            commands::public_projection,
            commands::public_export,
            commands::public_preview,
            commands::deep_link_initial,
            commands::deep_link_register,
            commands::vault_snapshot,
            commands::vault_action,
            commands::vault_files,
            commands::vault_pick_attachment,
            commands::vault_verify,
            commands::music_snapshot,
            commands::music_action,
            commands::music_status,
            commands::module_registry_get,
            commands::module_registry_save,
            commands::app_info,
            commands::insights_snapshot,
            commands::insights_action,
            commands::insights_review,
            commands::insights_month,
            commands::insights_memories,
            commands::coding_snapshot,
            commands::coding_action,
            commands::database_status,
            commands::today_snapshot,
            commands::today_action,
            commands::calendar_snapshot,
            commands::roadmap_snapshot,
            commands::roadmap_action,
            commands::journal_page,
            commands::journal_get,
            commands::journal_save,
            commands::journal_trash,
            commands::journal_attachments,
            commands::journal_pick_attachment,
            commands::journal_link,
            commands::journal_verify,
            commands::journal_open_attachment,
            commands::capture_open,
            commands::capture_close,
            commands::capture_status,
            commands::capture_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("Lumen runtime failed");
}
