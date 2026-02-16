use tauri::State;
use crate::state::app_state::AppState;
use crate::network::ftp_client::{FtpFileInfo, FtpClient};
use std::path::PathBuf;

#[tauri::command]
pub async fn ftp_connect(
    _state: State<'_, AppState>,
    _host: String,
    _port: Option<u16>,
    _username: Option<String>,
    _password: Option<String>,
    _use_tls: Option<bool>,
) -> Result<(), String> {
    // FTP client is now created per-request in other commands
    // This is a placeholder for compatibility
    Ok(())
}

#[tauri::command]
pub async fn ftp_disconnect(
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // FTP client is now created per-request
    // This is a placeholder for compatibility
    Ok(())
}

#[tauri::command]
pub async fn ftp_list_files(
    _state: State<'_, AppState>,
    url: String,
) -> Result<Vec<FtpFileInfo>, String> {
    // Parse FTP URL and create client
    let (client, path) = FtpClient::from_url(&url)
        .map_err(|e| format!("Failed to parse FTP URL: {}", e))?;
    
    // List directory contents
    client.list_directory(&path)
        .await
        .map_err(|e| format!("Failed to list directory: {}", e))
}

#[tauri::command]
pub async fn ftp_download_file(
    _state: State<'_, AppState>,
    url: String,
    local_path: String,
    resume: Option<bool>,
) -> Result<u64, String> {
    // Parse FTP URL and create client
    let (client, remote_path) = FtpClient::from_url(&url)
        .map_err(|e| format!("Failed to parse FTP URL: {}", e))?;
    
    let local_path_buf = PathBuf::from(local_path);
    
    // Check if we should resume
    let resume_from = if resume.unwrap_or(false) && local_path_buf.exists() {
        tokio::fs::metadata(&local_path_buf)
            .await
            .ok()
            .map(|m| m.len())
    } else {
        None
    };
    
    // Download the file
    client.download_file(&remote_path, &local_path_buf, resume_from)
        .await
        .map_err(|e| format!("FTP download failed: {}", e))
}

#[tauri::command]
pub async fn ftp_get_file_size(
    _state: State<'_, AppState>,
    url: String,
) -> Result<u64, String> {
    // Parse FTP URL and create client
    let (client, remote_path) = FtpClient::from_url(&url)
        .map_err(|e| format!("Failed to parse FTP URL: {}", e))?;
    
    // Get file info
    let file_info = client.get_file_info(&remote_path)
        .await
        .map_err(|e| format!("Failed to get file info: {}", e))?;
    
    file_info.file_size
        .ok_or_else(|| "File size not available".to_string())
}

#[tauri::command]
pub async fn ftp_upload_file(
    _state: State<'_, AppState>,
    local_path: String,
    url: String,
) -> Result<(), String> {
    // Upload not yet implemented - requires additional FtpClient methods
    Err("FTP upload not yet implemented".to_string())
}
