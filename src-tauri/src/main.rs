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
use commands::download_commands;

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

            app.manage(app_state);

            // Setup system tray
            services::tray_service::setup_tray(app)?;

            // Start clipboard monitor
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                services::clipboard_service::start_monitoring(handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Download commands
            download_commands::add_download,
            download_commands::pause_download,
            download_commands::resume_download,
            download_commands::cancel_download,
            download_commands::remove_download,
            download_commands::retry_download,
            download_commands::get_all_downloads,
            download_commands::get_file_info,
            download_commands::add_batch_downloads,
            download_commands::get_download_progress,
            download_commands::pause_all,
            download_commands::resume_all,
            download_commands::cancel_all,
            download_commands::open_file,
            download_commands::open_file_location,
            download_commands::get_global_stats,
            download_commands::set_speed_limit,
            download_commands::get_queue_info,
            download_commands::set_max_concurrent,
            // YouTube/video download commands
            download_commands::check_ytdlp_installed,
            download_commands::get_video_info,
            download_commands::get_video_qualities,
            download_commands::check_is_playlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
