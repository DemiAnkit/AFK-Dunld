// Commands for logging and monitoring
use tauri::State;
use crate::state::app_state::AppState;
use crate::utils::logging::{LogEntry, LogLevel, DownloadHistoryEntry, PerformanceMetrics};

#[tauri::command]
pub async fn get_logs(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<LogEntry>, String> {
    Ok(state.logger.get_logs(limit).await)
}

#[tauri::command]
pub async fn get_logs_by_level(
    state: State<'_, AppState>,
    level: String,
) -> Result<Vec<LogEntry>, String> {
    let log_level = match level.as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => return Err("Invalid log level".to_string()),
    };

    Ok(state.logger.get_logs_by_level(log_level).await)
}

#[tauri::command]
pub async fn get_logs_by_category(
    state: State<'_, AppState>,
    category: String,
) -> Result<Vec<LogEntry>, String> {
    Ok(state.logger.get_logs_by_category(&category).await)
}

#[tauri::command]
pub async fn get_logger_download_history(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<DownloadHistoryEntry>, String> {
    Ok(state.logger.get_history(limit).await)
}

#[tauri::command]
pub async fn get_performance_metrics(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<PerformanceMetrics>, String> {
    Ok(state.logger.get_metrics(limit).await)
}

#[tauri::command]
pub async fn clear_logs(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.logger.clear_logs().await;
    Ok(())
}

#[tauri::command]
pub async fn clear_logger_download_history(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.logger.clear_history().await;
    Ok(())
}
