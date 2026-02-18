// Native Messaging Host for Browser Extension Communication
// Implements Chrome/Firefox Native Messaging protocol

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use tauri::{AppHandle, Manager, Emitter};
use crate::state::app_state::AppState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NativeMessage {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "add_download")]
    AddDownload {
        url: String,
        referrer: Option<String>,
        filename: Option<String>,
        timestamp: Option<i64>,
    },
    #[serde(rename = "get_status")]
    GetStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NativeResponse {
    #[serde(rename = "pong")]
    Pong {
        version: String,
        app_name: String,
    },
    #[serde(rename = "download_added")]
    DownloadAdded {
        success: bool,
        download_id: Option<String>,
        error: Option<String>,
    },
    #[serde(rename = "status")]
    Status {
        active_downloads: usize,
        total_speed: f64,
    },
    #[serde(rename = "error")]
    Error {
        message: String,
    },
}

/// Read a message from stdin using Chrome Native Messaging protocol
/// Format: 4-byte message length (little-endian) followed by JSON message
pub fn read_message() -> io::Result<NativeMessage> {
    let mut length_bytes = [0u8; 4];
    io::stdin().read_exact(&mut length_bytes)?;
    
    let length = u32::from_le_bytes(length_bytes) as usize;
    
    if length == 0 || length > 1024 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid message length",
        ));
    }
    
    let mut buffer = vec![0u8; length];
    io::stdin().read_exact(&mut buffer)?;
    
    let message: NativeMessage = serde_json::from_slice(&buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(message)
}

/// Write a response to stdout using Chrome Native Messaging protocol
pub fn write_response(response: &NativeResponse) -> io::Result<()> {
    let json = serde_json::to_string(response)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    let length = json.len() as u32;
    let length_bytes = length.to_le_bytes();
    
    io::stdout().write_all(&length_bytes)?;
    io::stdout().write_all(json.as_bytes())?;
    io::stdout().flush()?;
    
    Ok(())
}

/// Handle a native messaging message
pub async fn handle_message(
    message: NativeMessage,
    app_handle: &AppHandle,
) -> NativeResponse {
    match message {
        NativeMessage::Ping => NativeResponse::Pong {
            version: env!("CARGO_PKG_VERSION").to_string(),
            app_name: "AFK-Dunld".to_string(),
        },
        
        NativeMessage::AddDownload {
            url,
            referrer,
            filename,
            ..
        } => {
            // Get app state
            let state = app_handle.state::<AppState>();
            let state_clone = state.inner().clone();
            
            // Add download
            match crate::commands::download_commands::add_download_internal(
                url.clone(),
                None, // save_path - use default
                filename,
                referrer,
                state_clone,
            ).await {
                Ok(download_id) => {
                    // Send notification
                    let _ = app_handle.emit("download-added", &download_id);
                    
                    NativeResponse::DownloadAdded {
                        success: true,
                        download_id: Some(download_id),
                        error: None,
                    }
                }
                Err(e) => NativeResponse::DownloadAdded {
                    success: false,
                    download_id: None,
                    error: Some(e.to_string()),
                },
            }
        }
        
        NativeMessage::GetStatus => {
            let state = app_handle.state::<AppState>();
            let downloads = state.db.get_all_downloads().await.unwrap_or_default();
            
            use crate::core::download_task::DownloadStatus;
            
            let active_downloads = downloads.iter()
                .filter(|d| matches!(d.status, DownloadStatus::Downloading | DownloadStatus::Queued))
                .count();
            
            let total_speed: f64 = downloads.iter()
                .filter(|d| d.status == DownloadStatus::Downloading)
                .map(|d| d.speed)
                .sum();
            
            NativeResponse::Status {
                active_downloads,
                total_speed,
            }
        }
    }
}

/// Run the native messaging host (stdio mode)
pub async fn run_native_messaging_host(app_handle: AppHandle) -> io::Result<()> {
    tracing::info!("Native messaging host started");
    
    loop {
        match read_message() {
            Ok(message) => {
                tracing::debug!("Received message: {:?}", message);
                
                let response = handle_message(message, &app_handle).await;
                
                if let Err(e) = write_response(&response) {
                    tracing::error!("Failed to write response: {}", e);
                    break;
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    tracing::info!("Native messaging host connection closed");
                    break;
                }
                
                tracing::error!("Failed to read message: {}", e);
                let error_response = NativeResponse::Error {
                    message: e.to_string(),
                };
                let _ = write_response(&error_response);
            }
        }
    }
    
    Ok(())
}
