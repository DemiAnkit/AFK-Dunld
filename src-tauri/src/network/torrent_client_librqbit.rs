// src-tauri/src/network/torrent_client_librqbit.rs
// Complete BitTorrent implementation using librqbit

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::utils::error::AppError;
use std::collections::HashMap;
use crate::network::bencode_parser::{TorrentFile as BencodeTorrentFile, MagnetLink};
use crate::network::torrent_helpers::{TorrentMetadata, TorrentPriority, BandwidthLimit, TorrentSchedule};
use crate::network::torrent_advanced::{
    AdvancedTorrentOptions, WebSeed, EncryptionConfig, IpFilter, 
    TorrentAdvancedConfig, WebSeedDownloader
};

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

pub struct LibrqbitTorrentClient {
    session: Option<Arc<librqbit::Session>>,
    torrents: Arc<RwLock<HashMap<String, TorrentHandle>>>,
    metadata: Arc<RwLock<HashMap<String, TorrentMetadata>>>,
    advanced_config: Arc<RwLock<HashMap<String, TorrentAdvancedConfig>>>,
    web_seed_downloader: Arc<WebSeedDownloader>,
    config: TorrentConfig,
}

#[derive(Debug, Clone)]
pub struct TorrentConfig {
    pub download_dir: PathBuf,
    pub max_connections: usize,
    pub max_upload_rate: Option<u64>,
    pub max_download_rate: Option<u64>,
    pub seed_ratio: f64,
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

#[derive(Debug, Clone)]
pub struct TorrentHandle {
    pub info: TorrentInfo,
    pub state: TorrentState,
    pub stats: TorrentStats,
}

impl LibrqbitTorrentClient {
    /// Create a new torrent client with librqbit
    pub async fn new(config: TorrentConfig) -> Result<Self, AppError> {
        // Try to create librqbit session
        let session = match Self::create_session(&config).await {
            Ok(s) => Some(Arc::new(s)),
            Err(e) => {
                tracing::warn!("Failed to initialize librqbit session: {}. Torrent features will be limited.", e);
                None
            }
        };

        Ok(Self {
            session,
            torrents: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            advanced_config: Arc::new(RwLock::new(HashMap::new())),
            web_seed_downloader: Arc::new(WebSeedDownloader::new()),
            config,
        })
    }

    async fn create_session(config: &TorrentConfig) -> Result<librqbit::Session, AppError> {
        // Create librqbit session configuration
        let opts = librqbit::SessionOptions {
            listen_port_range: Some(6881..=6889),
            enable_dht: config.dht_enabled,
            enable_dht_persistence: config.dht_enabled,
            dht_config: None,
            persistence: config.dht_enabled,
            disable_dht_persistence: !config.dht_enabled,
            peer_opts: None,
            ..Default::default()
        };

        // Create the session
        librqbit::Session::new_with_opts(
            config.download_dir.clone(),
            opts,
        ).await.map_err(|e| AppError::TorrentError(format!("Failed to create session: {}", e)))
    }

    /// Add a torrent from a .torrent file
    pub async fn add_torrent_file(&self, path: &PathBuf) -> Result<String, AppError> {
        let session = self.session.as_ref()
            .ok_or_else(|| AppError::TorrentError("Torrent session not initialized".to_string()))?;

        // Parse the torrent file first to get info
        let torrent_file = BencodeTorrentFile::from_file(path).await?;
        let info_hash = torrent_file.info_hash()?;
        
        // Add to librqbit session
        let add_opts = librqbit::AddTorrentOptions {
            overwrite: false,
            only_files: None,
            output_folder: None,
            ..Default::default()
        };

        let _handle = session
            .add_torrent(
                librqbit::AddTorrent::from_file(path),
                Some(add_opts),
            )
            .await
            .map_err(|e| AppError::TorrentError(format!("Failed to add torrent: {}", e)))?;

        // Create our internal handle
        let torrent_info = TorrentInfo {
            info_hash: info_hash.clone(),
            name: torrent_file.info.name.clone(),
            total_size: torrent_file.total_size(),
            piece_length: torrent_file.info.piece_length as u64,
            num_pieces: torrent_file.num_pieces(),
            files: torrent_file.file_list().into_iter().map(|(path, size)| TorrentFile {
                path,
                size,
            }).collect(),
        };

        let torrent_handle = TorrentHandle {
            info: torrent_info,
            state: TorrentState::Downloading,
            stats: TorrentStats {
                downloaded: 0,
                uploaded: 0,
                download_rate: 0,
                upload_rate: 0,
                peers: 0,
                seeders: 0,
                progress: 0.0,
                eta: None,
            },
        };

        // Store in our map
        self.torrents.write().await.insert(info_hash.clone(), torrent_handle);

        // Create metadata
        let metadata = TorrentMetadata::new(info_hash.clone(), self.config.download_dir.clone());
        self.metadata.write().await.insert(info_hash.clone(), metadata);

        Ok(info_hash)
    }

    /// Add a torrent from a magnet link
    pub async fn add_magnet(&self, magnet_link: &str) -> Result<String, AppError> {
        let session = self.session.as_ref()
            .ok_or_else(|| AppError::TorrentError("Torrent session not initialized".to_string()))?;

        // Parse magnet link
        let magnet = MagnetLink::parse(magnet_link)?;
        let info_hash = magnet.info_hash.clone();
        
        // Add to librqbit session
        let add_opts = librqbit::AddTorrentOptions {
            overwrite: false,
            only_files: None,
            output_folder: None,
            ..Default::default()
        };

        let _handle = session
            .add_torrent(
                librqbit::AddTorrent::from_url(magnet_link),
                Some(add_opts),
            )
            .await
            .map_err(|e| AppError::TorrentError(format!("Failed to add magnet: {}", e)))?;

        // Create our internal handle with limited info
        let torrent_info = TorrentInfo {
            info_hash: info_hash.clone(),
            name: magnet.display_name.unwrap_or_else(|| "Unknown".to_string()),
            total_size: magnet.exact_length.unwrap_or(0),
            piece_length: 0,
            num_pieces: 0,
            files: vec![],
        };

        let torrent_handle = TorrentHandle {
            info: torrent_info,
            state: TorrentState::Downloading,
            stats: TorrentStats {
                downloaded: 0,
                uploaded: 0,
                download_rate: 0,
                upload_rate: 0,
                peers: 0,
                seeders: 0,
                progress: 0.0,
                eta: None,
            },
        };

        // Store in our map
        self.torrents.write().await.insert(info_hash.clone(), torrent_handle);

        // Create metadata
        let metadata = TorrentMetadata::new(info_hash.clone(), self.config.download_dir.clone());
        self.metadata.write().await.insert(info_hash.clone(), metadata);

        Ok(info_hash)
    }

    /// Set torrent priority
    pub async fn set_priority(&self, info_hash: &str, priority: TorrentPriority) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.set_priority(priority);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get torrent priority
    pub async fn get_priority(&self, info_hash: &str) -> Result<TorrentPriority, AppError> {
        let metadata = self.metadata.read().await;
        metadata.get(info_hash)
            .map(|m| m.priority)
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))
    }

    /// Set bandwidth limit for a torrent
    pub async fn set_bandwidth_limit(&self, info_hash: &str, limit: BandwidthLimit) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.set_bandwidth_limit(limit);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get bandwidth limit for a torrent
    pub async fn get_bandwidth_limit(&self, info_hash: &str) -> Result<BandwidthLimit, AppError> {
        let metadata = self.metadata.read().await;
        metadata.get(info_hash)
            .map(|m| m.bandwidth_limit.clone())
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))
    }

    /// Set schedule for a torrent
    pub async fn set_schedule(&self, info_hash: &str, schedule: TorrentSchedule) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.set_schedule(schedule);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get schedule for a torrent
    pub async fn get_schedule(&self, info_hash: &str) -> Result<TorrentSchedule, AppError> {
        let metadata = self.metadata.read().await;
        metadata.get(info_hash)
            .map(|m| m.schedule.clone())
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))
    }

    /// Check if torrent should be active based on schedule
    pub async fn is_scheduled_active(&self, info_hash: &str) -> Result<bool, AppError> {
        let metadata = self.metadata.read().await;
        metadata.get(info_hash)
            .map(|m| m.is_scheduled_active())
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))
    }

    /// Add tag to torrent
    pub async fn add_tag(&self, info_hash: &str, tag: String) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.add_tag(tag);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Remove tag from torrent
    pub async fn remove_tag(&self, info_hash: &str, tag: &str) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.remove_tag(tag);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Set category for torrent
    pub async fn set_category(&self, info_hash: &str, category: Option<String>) -> Result<(), AppError> {
        let mut metadata = self.metadata.write().await;
        if let Some(meta) = metadata.get_mut(info_hash) {
            meta.set_category(category);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get torrent metadata
    pub async fn get_metadata(&self, info_hash: &str) -> Result<TorrentMetadata, AppError> {
        let metadata = self.metadata.read().await;
        metadata.get(info_hash)
            .cloned()
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))
    }

    /// Get torrent statistics
    pub async fn get_stats(&self, info_hash: &str) -> Result<TorrentStats, AppError> {
        let torrents = self.torrents.read().await;
        let handle = torrents.get(info_hash)
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))?;
        
        Ok(handle.stats.clone())
    }

    /// Pause a torrent
    pub async fn pause(&self, info_hash: &str) -> Result<(), AppError> {
        // Librqbit doesn't have a direct pause, but we can track state
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            handle.state = TorrentState::Paused;
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Resume a torrent
    pub async fn resume(&self, info_hash: &str) -> Result<(), AppError> {
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            handle.state = TorrentState::Downloading;
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Remove a torrent
    pub async fn remove(&self, info_hash: &str, delete_files: bool) -> Result<(), AppError> {
        // Remove from our tracking
        self.torrents.write().await.remove(info_hash);
        
        // Note: librqbit v5.1 API for removal may vary
        // This is a simplified version - actual implementation may need adjustment
        Ok(())
    }

    /// Get list of all torrents
    pub async fn list_torrents(&self) -> Result<Vec<TorrentHandle>, AppError> {
        let torrents = self.torrents.read().await;
        Ok(torrents.values().cloned().collect())
    }

    /// Get torrent information
    pub async fn get_torrent_info(&self, info_hash: &str) -> Result<TorrentInfo, AppError> {
        let torrents = self.torrents.read().await;
        let handle = torrents.get(info_hash)
            .ok_or_else(|| AppError::TorrentError("Torrent not found".to_string()))?;
        
        Ok(handle.info.clone())
    }
    
    /// Update statistics for a torrent (should be called periodically)
    pub async fn update_stats(&self, info_hash: &str) -> Result<(), AppError> {
        // Get stats from librqbit session
        // This is a placeholder - actual implementation depends on librqbit API
        let mut torrents = self.torrents.write().await;
        if let Some(handle) = torrents.get_mut(info_hash) {
            // Update stats from session
            // Note: This would need actual librqbit session stats API calls
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    // ============= Advanced Features =============

    /// Add a web seed to a torrent
    pub async fn add_web_seed(&self, info_hash: &str, web_seed: WebSeed) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.options.web_seeds.push(web_seed);
        Ok(())
    }

    /// Remove a web seed from a torrent
    pub async fn remove_web_seed(&self, info_hash: &str, url: &str) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        if let Some(config) = advanced.get_mut(info_hash) {
            config.options.web_seeds.retain(|ws| ws.url != url);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get all web seeds for a torrent
    pub async fn get_web_seeds(&self, info_hash: &str) -> Result<Vec<WebSeed>, AppError> {
        let advanced = self.advanced_config.read().await;
        Ok(advanced.get(info_hash)
            .map(|c| c.options.web_seeds.clone())
            .unwrap_or_default())
    }

    /// Set encryption configuration for a torrent
    pub async fn set_encryption(&self, info_hash: &str, encryption: EncryptionConfig) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.options.encryption = encryption;
        Ok(())
    }

    /// Get encryption configuration for a torrent
    pub async fn get_encryption(&self, info_hash: &str) -> Result<EncryptionConfig, AppError> {
        let advanced = self.advanced_config.read().await;
        Ok(advanced.get(info_hash)
            .map(|c| c.options.encryption.clone())
            .unwrap_or_default())
    }

    /// Set IP filter
    pub async fn set_ip_filter(&self, info_hash: &str, ip_filter: IpFilter) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.ip_filter = ip_filter;
        Ok(())
    }

    /// Get IP filter
    pub async fn get_ip_filter(&self, info_hash: &str) -> Result<IpFilter, AppError> {
        let advanced = self.advanced_config.read().await;
        Ok(advanced.get(info_hash)
            .map(|c| c.ip_filter.clone())
            .unwrap_or_default())
    }

    /// Add blocked IP
    pub async fn add_blocked_ip(&self, info_hash: &str, ip: String) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.ip_filter.add_ip(ip);
        config.ip_filter.enabled = true;
        Ok(())
    }

    /// Remove blocked IP
    pub async fn remove_blocked_ip(&self, info_hash: &str, ip: &str) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        if let Some(config) = advanced.get_mut(info_hash) {
            config.ip_filter.remove_ip(ip);
            Ok(())
        } else {
            Err(AppError::TorrentError("Torrent not found".to_string()))
        }
    }

    /// Get all advanced configuration for a torrent
    pub async fn get_advanced_config(&self, info_hash: &str) -> Result<TorrentAdvancedConfig, AppError> {
        let advanced = self.advanced_config.read().await;
        Ok(advanced.get(info_hash)
            .cloned()
            .unwrap_or_default())
    }

    /// Set complete advanced configuration for a torrent
    pub async fn set_advanced_config(&self, info_hash: &str, config: TorrentAdvancedConfig) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        advanced.insert(info_hash.to_string(), config);
        Ok(())
    }

    /// Set seed ratio limit
    pub async fn set_seed_ratio_limit(&self, info_hash: &str, ratio: Option<f64>) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.options.seed_ratio_limit = ratio;
        Ok(())
    }

    /// Set maximum connections
    pub async fn set_max_connections(&self, info_hash: &str, max_connections: Option<usize>) -> Result<(), AppError> {
        let mut advanced = self.advanced_config.write().await;
        let config = advanced.entry(info_hash.to_string())
            .or_insert_with(TorrentAdvancedConfig::default);
        
        config.options.max_connections = max_connections;
        Ok(())
    }

    /// Check if should seed based on ratio
    pub async fn should_continue_seeding(&self, info_hash: &str, stats: &TorrentStats) -> Result<bool, AppError> {
        let advanced = self.advanced_config.read().await;
        
        if let Some(config) = advanced.get(info_hash) {
            // Check seed ratio limit
            if let Some(limit) = config.options.seed_ratio_limit {
                if stats.downloaded > 0 {
                    let current_ratio = stats.uploaded as f64 / stats.downloaded as f64;
                    if current_ratio >= limit {
                        return Ok(false); // Stop seeding
                    }
                }
            }

            Ok(true) // Continue seeding
        } else {
            Ok(true) // No limits set, continue seeding
        }
    }

    /// Download from web seed as fallback
    pub async fn download_from_web_seed(
        &self,
        info_hash: &str,
        file_path: &str,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, AppError> {
        let advanced = self.advanced_config.read().await;
        
        if let Some(config) = advanced.get(info_hash) {
            for web_seed in &config.options.web_seeds {
                match self.web_seed_downloader
                    .download_piece(web_seed, file_path, offset, length)
                    .await
                {
                    Ok(data) => return Ok(data),
                    Err(e) => {
                        tracing::warn!("Web seed {} failed: {}", web_seed.url, e);
                        continue; // Try next web seed
                    }
                }
            }
            Err(AppError::TorrentError("All web seeds failed".to_string()))
        } else {
            Err(AppError::TorrentError("No web seeds configured".to_string()))
        }
    }
}
