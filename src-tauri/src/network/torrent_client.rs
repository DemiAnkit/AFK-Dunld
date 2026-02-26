use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::utils::error::AppError;
use crate::network::torrent_client_librqbit::LibrqbitTorrentClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub info_hash: String,
    pub name: String,
    pub total_size: u64,
    pub piece_length: u64,
    pub num_pieces: u64,
    pub files: Vec<TorrentFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFile {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentStats {
    pub downloaded: u64,
    pub uploaded: u64,
    pub download_rate: u64,
    pub upload_rate: u64,
    pub peers: usize,
    pub seeders: usize,
    pub progress: f64,
    pub eta: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TorrentState {
    Downloading,
    Seeding,
    Paused,
    Checking,
    Error(String),
}

pub struct TorrentClient {
    backend: Arc<LibrqbitTorrentClient>,
    torrents: Arc<RwLock<std::collections::HashMap<String, TorrentHandle>>>,
    config: TorrentConfig,
}

#[derive(Debug, Clone)]
pub struct TorrentConfig {
    pub download_dir: PathBuf,
    pub max_connections: usize,
    pub max_upload_rate: Option<u64>, // bytes per second
    pub max_download_rate: Option<u64>, // bytes per second
    pub seed_ratio: f64, // Stop seeding after this ratio
    pub dht_enabled: bool,
    pub pex_enabled: bool,
}

impl Default for TorrentConfig {
    fn default() -> Self {
        Self {
            download_dir: PathBuf::from("downloads"),
            max_connections: 200,
            max_upload_rate: None,
            max_download_rate: None,
            seed_ratio: 2.0,
            dht_enabled: true,
            pex_enabled: true,
        }
    }
}

struct TorrentHandle {
    info: TorrentInfo,
    state: TorrentState,
    stats: TorrentStats,
    save_path: PathBuf,
}

impl TorrentClient {
    pub async fn new(config: TorrentConfig) -> Result<Self, AppError> {
        // Convert to librqbit config
        let librqbit_config = crate::network::torrent_client_librqbit::TorrentConfig {
            download_dir: config.download_dir.clone(),
            max_connections: config.max_connections,
            max_upload_rate: config.max_upload_rate,
            max_download_rate: config.max_download_rate,
            seed_ratio: config.seed_ratio,
            dht_enabled: config.dht_enabled,
            pex_enabled: config.pex_enabled,
        };

        let backend = LibrqbitTorrentClient::new(librqbit_config).await?;

        Ok(Self {
            backend: Arc::new(backend),
            torrents: Arc::new(RwLock::new(std::collections::HashMap::new())),
            config,
        })
    }

    /// Add a torrent from a .torrent file
    pub async fn add_torrent_file(&self, path: &PathBuf) -> Result<String, AppError> {
        // Use librqbit backend to add torrent
        let info_hash = self.backend.add_torrent_file(path).await?;
        
        // Get info from backend
        let info = self.backend.get_torrent_info(&info_hash).await?;
        let stats = self.backend.get_stats(&info_hash).await?;
        
        // Create our handle
        let handle = TorrentHandle {
            info,
            state: TorrentState::Downloading,
            stats,
            save_path: self.config.download_dir.clone(),
        };

        let mut torrents = self.torrents.write().await;
        torrents.insert(info_hash.clone(), handle);

        Ok(info_hash)
    }

    /// Add a torrent from a magnet link
    pub async fn add_magnet(&self, magnet_link: &str) -> Result<String, AppError> {
        // Use librqbit backend to add magnet
        let info_hash = self.backend.add_magnet(magnet_link).await?;
        
        // Get info from backend
        let info = self.backend.get_torrent_info(&info_hash).await?;
        let stats = self.backend.get_stats(&info_hash).await?;
        
        // Create our handle
        let handle = TorrentHandle {
            info,
            state: TorrentState::Downloading,
            stats,
            save_path: self.config.download_dir.clone(),
        };

        let mut torrents = self.torrents.write().await;
        torrents.insert(info_hash.clone(), handle);

        Ok(info_hash)
    }

    /// Get torrent statistics
    pub async fn get_stats(&self, info_hash: &str) -> Result<TorrentStats, AppError> {
        let torrents = self.torrents.read().await;
        torrents
            .get(info_hash)
            .map(|h| h.stats.clone())
            .ok_or_else(|| AppError::NotFound(format!("Torrent {} not found", info_hash)))
    }

    /// Get torrent state
    pub async fn get_state(&self, info_hash: &str) -> Result<TorrentState, AppError> {
        let torrents = self.torrents.read().await;
        torrents
            .get(info_hash)
            .map(|h| h.state.clone())
            .ok_or_else(|| AppError::NotFound(format!("Torrent {} not found", info_hash)))
    }

    /// Pause a torrent
    pub async fn pause(&self, info_hash: &str) -> Result<(), AppError> {
        // Pause in backend
        self.backend.pause(info_hash).await?;
        
        // Update local state
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            handle.state = TorrentState::Paused;
        }
        
        Ok(())
    }

    /// Resume a torrent
    pub async fn resume(&self, info_hash: &str) -> Result<(), AppError> {
        // Resume in backend
        self.backend.resume(info_hash).await?;
        
        // Update local state
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            handle.state = TorrentState::Downloading;
        }
        
        Ok(())
    }

    /// Remove a torrent
    pub async fn remove(&self, info_hash: &str, delete_files: bool) -> Result<(), AppError> {
        // Remove from backend
        self.backend.remove(info_hash, delete_files).await?;
        
        // Remove from local tracking
        self.torrents.write().await.remove(info_hash);
        
        Ok(())
    }

    /// Get list of all torrents
    pub async fn list_torrents(&self) -> Result<Vec<TorrentInfo>, AppError> {
        let torrents = self.torrents.read().await;
        Ok(torrents.values().map(|h| h.info.clone()).collect())
    }
    
    /// Update statistics from backend (should be called periodically)
    pub async fn update_stats(&self, info_hash: &str) -> Result<(), AppError> {
        // Get fresh stats from backend
        let stats = self.backend.get_stats(info_hash).await?;
        
        // Update local cache
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            handle.stats = stats;
        }
        
        Ok(())
    }
}
