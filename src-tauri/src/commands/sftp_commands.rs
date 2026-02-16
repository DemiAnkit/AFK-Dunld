use tauri::State;
use crate::state::app_state::AppState;
use crate::network::sftp_client::{SftpFileInfo, SftpClient};
use std::path::PathBuf;

#[tauri::command]
pub async fn sftp_connect(
    _state: State<'_, AppState>,
    _host: String,
    _port: Option<u16>,
    _username: String,
    _password: Option<String>,
    _key_path: Option<String>,
) -> Result<(), String> {
    // SFTP client is created per-request in other commands
    // This is a placeholder for compatibility
    Ok(())
}

#[tauri::command]
pub async fn sftp_disconnect(
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // SFTP client is created per-request
    // This is a placeholder for compatibility
    Ok(())
}

#[tauri::command]
pub async fn sftp_list_files(
    _state: State<'_, AppState>,
    url: String,
    password: Option<String>,
    key_path: Option<String>,
) -> Result<Vec<SftpFileInfo>, String> {
    // Parse SFTP URL and create client
    let key_path_buf = key_path.map(PathBuf::from);
    let (client, path) = SftpClient::from_url(&url, password, key_path_buf)
        .map_err(|e| format!("Failed to parse SFTP URL: {}", e))?;
    
    // List directory contents
    client.list_directory(&path)
        .await
        .map_err(|e| format!("Failed to list directory: {}", e))
}

#[tauri::command]
pub async fn sftp_download_file(
    _state: State<'_, AppState>,
    url: String,
    local_path: String,
    password: Option<String>,
    key_path: Option<String>,
    resume: Option<bool>,
) -> Result<u64, String> {
    // Parse SFTP URL and create client
    let key_path_buf = key_path.map(PathBuf::from);
    let (client, remote_path) = SftpClient::from_url(&url, password, key_path_buf)
        .map_err(|e| format!("Failed to parse SFTP URL: {}", e))?;
    
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
        .map_err(|e| format!("SFTP download failed: {}", e))
}

#[tauri::command]
pub async fn sftp_get_file_size(
    _state: State<'_, AppState>,
    url: String,
    password: Option<String>,
    key_path: Option<String>,
) -> Result<u64, String> {
    // Parse SFTP URL and create client
    let key_path_buf = key_path.map(PathBuf::from);
    let (client, remote_path) = SftpClient::from_url(&url, password, key_path_buf)
        .map_err(|e| format!("Failed to parse SFTP URL: {}", e))?;
    
    // Get file info
    let file_info = client.get_file_info(&remote_path)
        .await
        .map_err(|e| format!("Failed to get file info: {}", e))?;
    
    file_info.file_size
        .ok_or_else(|| "File size not available".to_string())
}

#[tauri::command]
pub async fn sftp_upload_file(
    _state: State<'_, AppState>,
    local_path: String,
    url: String,
    password: Option<String>,
    key_path: Option<String>,
) -> Result<u64, String> {
    // Parse SFTP URL and create client
    let key_path_buf = key_path.map(PathBuf::from);
    let (client, remote_path) = SftpClient::from_url(&url, password, key_path_buf)
        .map_err(|e| format!("Failed to parse SFTP URL: {}", e))?;
    
    let local_path_buf = PathBuf::from(local_path);
    
    // Upload the file
    client.upload_file(&local_path_buf, &remote_path)
        .await
        .map_err(|e| format!("SFTP upload failed: {}", e))
}

#[tauri::command]
pub async fn sftp_get_file_info(
    _state: State<'_, AppState>,
    url: String,
    password: Option<String>,
    key_path: Option<String>,
) -> Result<SftpFileInfo, String> {
    // Parse SFTP URL and create client
    let key_path_buf = key_path.map(PathBuf::from);
    let (client, remote_path) = SftpClient::from_url(&url, password, key_path_buf)
        .map_err(|e| format!("Failed to parse SFTP URL: {}", e))?;
    
    // Get file info
    client.get_file_info(&remote_path)
        .await
        .map_err(|e| format!("Failed to get file info: {}", e))
}
