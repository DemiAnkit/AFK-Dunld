use crate::state::app_state::AppState;
use crate::core::download_task::DownloadTask;
use tauri::State;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryItem {
    pub id: String,
    pub url: String,
    pub file_name: String,
    pub total_size: Option<u64>,
    pub status: String,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub category: Option<String>,
    pub download_speed_avg: f64,
    pub download_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_downloads: usize,
    pub completed_downloads: usize,
    pub failed_downloads: usize,
    pub total_bytes_downloaded: u64,
    pub average_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFilter {
    pub status: Option<String>,
    pub category: Option<String>,
    pub limit: Option<usize>,
}

/// Get download history with optional filters
#[tauri::command]
pub async fn get_download_history(
    state: State<'_, AppState>,
    filter: Option<HistoryFilter>,
) -> Result<Vec<DownloadHistoryItem>, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut history: Vec<DownloadHistoryItem> = downloads
        .into_iter()
        .map(|task| DownloadHistoryItem {
            id: task.id.to_string(),
            url: task.url.clone(),
            file_name: task.file_name.clone(),
            total_size: task.total_size,
            status: format!("{:?}", task.status),
            completed_at: task.completed_at,
            created_at: task.created_at,
            category: task.category.clone(),
            download_speed_avg: task.speed,
            download_time: None,
        })
        .collect();
    
    // Apply filters
    if let Some(f) = filter {
        if let Some(status) = f.status {
            history.retain(|item| item.status.to_lowercase().contains(&status.to_lowercase()));
        }
        if let Some(category) = f.category {
            history.retain(|item| {
                item.category.as_ref().map(|c| c == &category).unwrap_or(false)
            });
        }
        if let Some(limit) = f.limit {
            history.truncate(limit);
        }
    }
    
    Ok(history)
}

/// Get history statistics
#[tauri::command]
pub async fn get_history_stats(state: State<'_, AppState>) -> Result<HistoryStats, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let total_downloads = downloads.len();
    let completed_downloads = downloads.iter()
        .filter(|d| format!("{:?}", d.status).contains("Completed"))
        .count();
    let failed_downloads = downloads.iter()
        .filter(|d| format!("{:?}", d.status).contains("Failed"))
        .count();
    
    let total_bytes_downloaded: u64 = downloads.iter()
        .filter_map(|d| d.total_size)
        .sum();
    
    let average_speed = if !downloads.is_empty() {
        downloads.iter().map(|d| d.speed).sum::<f64>() / downloads.len() as f64
    } else {
        0.0
    };
    
    Ok(HistoryStats {
        total_downloads,
        completed_downloads,
        failed_downloads,
        total_bytes_downloaded,
        average_speed,
    })
}

/// Clear download history (delete completed downloads)
#[tauri::command]
pub async fn clear_download_history(state: State<'_, AppState>) -> Result<usize, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut deleted_count = 0;
    
    // Only delete completed, failed, and cancelled downloads
    for download in downloads {
        match download.status {
            crate::core::download_task::DownloadStatus::Completed |
            crate::core::download_task::DownloadStatus::Failed |
            crate::core::download_task::DownloadStatus::Cancelled => {
                if state.db.delete_download(download.id).await.is_ok() {
                    deleted_count += 1;
                }
            },
            _ => {
                // Skip active downloads
            }
        }
    }
    
    Ok(deleted_count)
}

/// Delete a specific download from history
#[tauri::command]
pub async fn delete_download_from_history(
    state: State<'_, AppState>,
    download_id: String,
) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&download_id)
        .map_err(|e| format!("Invalid download ID: {}", e))?;
    
    state.db.delete_download(id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Delete multiple downloads from history
#[tauri::command]
pub async fn delete_downloads_bulk(
    state: State<'_, AppState>,
    download_ids: Vec<String>,
) -> Result<usize, String> {
    let mut deleted_count = 0;
    
    for id_str in download_ids {
        if let Ok(id) = uuid::Uuid::parse_str(&id_str) {
            if state.db.delete_download(id).await.is_ok() {
                deleted_count += 1;
            }
        }
    }
    
    Ok(deleted_count)
}

/// Clear history older than specified days
#[tauri::command]
pub async fn clear_old_history(
    state: State<'_, AppState>,
    days: i64,
) -> Result<usize, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let cutoff_date = chrono::Local::now().naive_local() - chrono::Duration::days(days);
    let mut deleted_count = 0;
    
    for download in downloads {
        // Only delete old completed/failed downloads
        if download.completed_at.is_some() || 
           matches!(download.status, 
                    crate::core::download_task::DownloadStatus::Failed | 
                    crate::core::download_task::DownloadStatus::Cancelled) {
            
            if download.created_at < cutoff_date {
                if state.db.delete_download(download.id).await.is_ok() {
                    deleted_count += 1;
                }
            }
        }
    }
    
    Ok(deleted_count)
}

/// Export history to JSON
#[tauri::command]
pub async fn export_history(
    state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    let history = get_download_history(state.clone(), None).await?;
    
    let json = serde_json::to_string_pretty(&history)
        .map_err(|e| e.to_string())?;
    
    std::fs::write(path, json)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
