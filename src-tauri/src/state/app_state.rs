use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::download_engine::DownloadEngine;
use crate::core::download_task::DownloadTask;
use crate::core::queue_manager::QueueManager;
use crate::database::db::Database;

/// Handle for an active download (used for cancellation)
pub struct ActiveDownload {
    pub cancel_token: tokio_util::sync::CancellationToken,
    pub task_handle: tokio::task::JoinHandle<()>,
    pub task: Arc<RwLock<DownloadTask>>,
}

/// Download handle type alias for backwards compatibility
pub type DownloadHandle = ActiveDownload;

/// Global application state managed by Tauri
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub engine: Arc<DownloadEngine>,
    pub queue: Arc<RwLock<QueueManager>>,
    pub active_downloads:
        Arc<RwLock<HashMap<Uuid, ActiveDownload>>>,
    pub download_dir: PathBuf,
}

impl AppState {
    pub async fn new(
        app_data_dir: PathBuf,
    ) -> Result<Self, crate::utils::error::DownloadError> {
        // Initialize database
        let db = Database::new(&app_data_dir).await?;
        db.run_migrations().await?;

        // Initialize download engine
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("Downloads")
            });

        let engine = Arc::new(DownloadEngine::new(
            None,       // No proxy by default
            None,       // No speed limit by default
            Some(download_dir.clone()),
        )?);

        let queue =
            Arc::new(RwLock::new(QueueManager::new(5)));

        Ok(Self {
            db,
            engine,
            queue,
            active_downloads: Arc::new(RwLock::new(
                HashMap::new(),
            )),
            download_dir,
        })
    }
}