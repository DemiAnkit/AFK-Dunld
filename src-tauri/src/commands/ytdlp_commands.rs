use crate::state::app_state::AppState;
use tauri::State;

/// Update yt-dlp to the latest version
#[tauri::command]
pub async fn update_ytdlp(state: State<'_, AppState>) -> Result<String, String> {
    state.ytdlp_manager
        .update()
        .await
        .map(|_| "yt-dlp updated successfully".to_string())
        .map_err(|e| e.to_string())
}

/// Get the current yt-dlp version
#[tauri::command]
pub async fn get_ytdlp_version(state: State<'_, AppState>) -> Result<String, String> {
    state.ytdlp_manager
        .get_version()
        .await
        .map_err(|e| e.to_string())
}

/// Get the bundled yt-dlp version
#[tauri::command]
pub async fn get_bundled_ytdlp_version(state: State<'_, AppState>) -> Result<String, String> {
    state.ytdlp_manager
        .get_bundled_version()
        .ok_or_else(|| "Bundled version information not available".to_string())
}
