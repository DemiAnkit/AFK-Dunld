// Structured Logging and Monitoring System
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// Log level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub category: String,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

impl LogEntry {
    pub fn new(level: LogLevel, category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            category: category.into(),
            message: message.into(),
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Download history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryEntry {
    pub id: String,
    pub url: String,
    pub file_name: String,
    pub total_size: Option<u64>,
    pub downloaded_size: u64,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u64>,
    pub average_speed: Option<u64>,
    pub error_message: Option<String>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: DateTime<Utc>,
    pub active_downloads: usize,
    pub total_download_speed: u64,
    pub total_upload_speed: u64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub disk_usage_mb: f64,
}

/// Logger with in-memory buffer
pub struct Logger {
    logs: Arc<RwLock<VecDeque<LogEntry>>>,
    history: Arc<RwLock<VecDeque<DownloadHistoryEntry>>>,
    metrics: Arc<RwLock<VecDeque<PerformanceMetrics>>>,
    max_logs: usize,
    max_history: usize,
    max_metrics: usize,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::new())),
            history: Arc::new(RwLock::new(VecDeque::new())),
            metrics: Arc::new(RwLock::new(VecDeque::new())),
            max_logs: 1000,
            max_history: 500,
            max_metrics: 100,
        }
    }

    /// Log a message
    pub async fn log(&self, entry: LogEntry) {
        // Also log to tracing
        match entry.level {
            LogLevel::Trace => tracing::trace!("[{}] {}", entry.category, entry.message),
            LogLevel::Debug => tracing::debug!("[{}] {}", entry.category, entry.message),
            LogLevel::Info => tracing::info!("[{}] {}", entry.category, entry.message),
            LogLevel::Warn => tracing::warn!("[{}] {}", entry.category, entry.message),
            LogLevel::Error => tracing::error!("[{}] {}", entry.category, entry.message),
        }

        let mut logs = self.logs.write().await;
        logs.push_back(entry);

        // Keep only max_logs entries
        while logs.len() > self.max_logs {
            logs.pop_front();
        }
    }

    /// Add to download history
    pub async fn add_history(&self, entry: DownloadHistoryEntry) {
        let mut history = self.history.write().await;
        history.push_back(entry);

        while history.len() > self.max_history {
            history.pop_front();
        }
    }

    /// Record performance metrics
    pub async fn record_metrics(&self, metrics: PerformanceMetrics) {
        let mut metrics_buffer = self.metrics.write().await;
        metrics_buffer.push_back(metrics);

        while metrics_buffer.len() > self.max_metrics {
            metrics_buffer.pop_front();
        }
    }

    /// Get recent logs
    pub async fn get_logs(&self, limit: Option<usize>) -> Vec<LogEntry> {
        let logs = self.logs.read().await;
        let limit = limit.unwrap_or(100).min(logs.len());
        logs.iter().rev().take(limit).cloned().collect()
    }

    /// Get download history
    pub async fn get_history(&self, limit: Option<usize>) -> Vec<DownloadHistoryEntry> {
        let history = self.history.read().await;
        let limit = limit.unwrap_or(50).min(history.len());
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get performance metrics
    pub async fn get_metrics(&self, limit: Option<usize>) -> Vec<PerformanceMetrics> {
        let metrics = self.metrics.read().await;
        let limit = limit.unwrap_or(20).min(metrics.len());
        metrics.iter().rev().take(limit).cloned().collect()
    }

    /// Get logs by level
    pub async fn get_logs_by_level(&self, level: LogLevel) -> Vec<LogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.level >= level)
            .cloned()
            .collect()
    }

    /// Get logs by category
    pub async fn get_logs_by_category(&self, category: &str) -> Vec<LogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.category == category)
            .cloned()
            .collect()
    }

    /// Clear all logs
    pub async fn clear_logs(&self) {
        self.logs.write().await.clear();
    }

    /// Clear history
    pub async fn clear_history(&self) {
        self.history.write().await.clear();
    }

    /// Clear metrics
    pub async fn clear_metrics(&self) {
        self.metrics.write().await.clear();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macros for logging
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $category:expr, $message:expr) => {
        $logger.log(LogEntry::new(LogLevel::Trace, $category, $message)).await
    };
    ($logger:expr, $category:expr, $message:expr, $metadata:expr) => {
        $logger.log(LogEntry::new(LogLevel::Trace, $category, $message).with_metadata($metadata)).await
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $category:expr, $message:expr) => {
        $logger.log(LogEntry::new(LogLevel::Debug, $category, $message)).await
    };
    ($logger:expr, $category:expr, $message:expr, $metadata:expr) => {
        $logger.log(LogEntry::new(LogLevel::Debug, $category, $message).with_metadata($metadata)).await
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $category:expr, $message:expr) => {
        $logger.log(LogEntry::new(LogLevel::Info, $category, $message)).await
    };
    ($logger:expr, $category:expr, $message:expr, $metadata:expr) => {
        $logger.log(LogEntry::new(LogLevel::Info, $category, $message).with_metadata($metadata)).await
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $category:expr, $message:expr) => {
        $logger.log(LogEntry::new(LogLevel::Warn, $category, $message)).await
    };
    ($logger:expr, $category:expr, $message:expr, $metadata:expr) => {
        $logger.log(LogEntry::new(LogLevel::Warn, $category, $message).with_metadata($metadata)).await
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $category:expr, $message:expr) => {
        $logger.log(LogEntry::new(LogLevel::Error, $category, $message)).await
    };
    ($logger:expr, $category:expr, $message:expr, $metadata:expr) => {
        $logger.log(LogEntry::new(LogLevel::Error, $category, $message).with_metadata($metadata)).await
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logger_basic() {
        let logger = Logger::new();
        
        let entry = LogEntry::new(LogLevel::Info, "test", "Test message");
        logger.log(entry).await;

        let logs = logger.get_logs(None).await;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Test message");
    }

    #[tokio::test]
    async fn test_logger_max_capacity() {
        let mut logger = Logger::new();
        logger.max_logs = 5;

        for i in 0..10 {
            let entry = LogEntry::new(LogLevel::Info, "test", format!("Message {}", i));
            logger.log(entry).await;
        }

        let logs = logger.get_logs(None).await;
        assert_eq!(logs.len(), 5);
    }

    #[tokio::test]
    async fn test_filter_by_level() {
        let logger = Logger::new();
        
        logger.log(LogEntry::new(LogLevel::Debug, "test", "Debug")).await;
        logger.log(LogEntry::new(LogLevel::Info, "test", "Info")).await;
        logger.log(LogEntry::new(LogLevel::Warn, "test", "Warn")).await;
        logger.log(LogEntry::new(LogLevel::Error, "test", "Error")).await;

        let errors = logger.get_logs_by_level(LogLevel::Error).await;
        assert_eq!(errors.len(), 1);

        let warnings_and_above = logger.get_logs_by_level(LogLevel::Warn).await;
        assert_eq!(warnings_and_above.len(), 2);
    }

    #[tokio::test]
    async fn test_filter_by_category() {
        let logger = Logger::new();
        
        logger.log(LogEntry::new(LogLevel::Info, "download", "Download started")).await;
        logger.log(LogEntry::new(LogLevel::Info, "upload", "Upload started")).await;
        logger.log(LogEntry::new(LogLevel::Info, "download", "Download completed")).await;

        let download_logs = logger.get_logs_by_category("download").await;
        assert_eq!(download_logs.len(), 2);
    }
}
