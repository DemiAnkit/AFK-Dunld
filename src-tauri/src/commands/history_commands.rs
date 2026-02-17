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

/// Clear download history
#[tauri::command]
pub async fn clear_download_history(state: State<'_, AppState>) -> Result<usize, String> {
    // For now, return 0 as we don't have delete_all_downloads method
    // TODO: Implement proper deletion when the database method is available
    Ok(0)
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
