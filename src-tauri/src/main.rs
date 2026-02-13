// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod network;
mod database;
mod commands;
mod services;
mod state;
mod utils;
mod events;

use tauri::Manager;
use state::app_state::AppState;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("super_downloader=debug")
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let app_state = tauri::async_runtime::block_on(async {
                AppState::new(app_data_dir).await.expect("Failed to initialize app state")
            });

            app.manage(app_state.clone());

            // Setup system tray
            services::tray_service::setup_tray(app)?;

            // Start clipboard monitor
            let handle = app.handle().clone();
            let _state_clone = app_state.clone();
            tauri::async_runtime::spawn(async move {
                services::clipboard_service::start_monitoring(handle).await;
            });

            // Start file watcher service
            let handle = app.handle().clone();
            services::file_watcher::FileWatcher::start(handle, app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Download commands
            commands::download_commands::add_download,
            commands::download_commands::pause_download,
            commands::download_commands::resume_download,
            commands::download_commands::cancel_download,
            commands::download_commands::remove_download,
            commands::download_commands::retry_download,
            commands::download_commands::get_all_downloads,
            commands::download_commands::get_file_info,
            commands::download_commands::add_batch_downloads,
            commands::download_commands::get_download_progress,
            commands::download_commands::pause_all,
            commands::download_commands::resume_all,
            commands::download_commands::cancel_all,
            commands::download_commands::open_file,
            commands::download_commands::open_file_location,
            commands::download_commands::get_global_stats,
            commands::download_commands::set_speed_limit,
            commands::download_commands::get_queue_info,
            commands::download_commands::set_max_concurrent,
            commands::download_commands::check_file_exists,
            commands::download_commands::get_file_size,
            // YouTube/video download commands
            commands::download_commands::check_ytdlp_installed,
            commands::download_commands::get_video_info,
            commands::download_commands::get_video_qualities,
            commands::download_commands::check_is_playlist,
            // Settings commands
            commands::settings_commands::get_settings,
            commands::settings_commands::get_setting,
            commands::settings_commands::update_settings,
            commands::settings_commands::reset_settings,
            // System commands
            commands::system_commands::get_system_info,
            commands::system_commands::check_disk_space,
            // Service commands
            services::clipboard_service::set_clipboard_monitoring,
            services::notification_service::set_notifications_enabled,
            services::notification_service::test_notification,
            services::tray_service::handle_tray_menu_click,
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
