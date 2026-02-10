// src-tauri/src/services/clipboard_service.rs

use tauri::AppHandle;

pub async fn start_monitoring(_app_handle: AppHandle) {
    // TODO: Implement clipboard monitoring
    // For now, just loop and sleep to prevent the function from returning
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
