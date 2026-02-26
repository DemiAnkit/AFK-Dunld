// src-tauri/src/network/torrent_advanced.rs
// Advanced torrent features: web seeds, encryption, DHT bootstrap

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::error::AppError;

/// Web seed configuration for HTTP/HTTPS fallback sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSeed {
    pub url: String,
    pub seed_type: WebSeedType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebSeedType {
    /// GetRight style - each file is downloaded from url/file_path
    GetRight,
    /// Web seed (BEP 19) - supports range requests
    WebSeed,
}

impl WebSeed {
    pub fn new(url: String, seed_type: WebSeedType) -> Self {
        Self { url, seed_type }
    }

    /// Construct URL for a specific file
    pub fn file_url(&self, file_path: &str) -> String {
        match self.seed_type {
            WebSeedType::GetRight => {
                format!("{}/{}", self.url.trim_end_matches('/'), file_path.trim_start_matches('/'))
            }
            WebSeedType::WebSeed => {
                self.url.clone()
            }
        }
    }
}

/// Protocol encryption settings (BEP 3, MSE/PE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Whether to enable protocol encryption
    pub enabled: bool,
    /// Encryption mode
    pub mode: EncryptionMode,
    /// Prefer encrypted connections
    pub prefer_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncryptionMode {
    /// Disabled - plaintext only
    Disabled,
    /// Enabled - encryption optional
    Enabled,
    /// Required - only encrypted connections
    Required,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: EncryptionMode::Enabled,
            prefer_encrypted: true,
        }
    }
}

impl EncryptionConfig {
    pub fn new(enabled: bool, mode: EncryptionMode, prefer_encrypted: bool) -> Self {
        Self {
            enabled,
            mode,
            prefer_encrypted,
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            mode: EncryptionMode::Disabled,
            prefer_encrypted: false,
        }
    }

    pub fn required() -> Self {
        Self {
            enabled: true,
            mode: EncryptionMode::Required,
            prefer_encrypted: true,
        }
    }
}

/// DHT bootstrap node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhtBootstrapNode {
    pub host: String,
    pub port: u16,
}

impl DhtBootstrapNode {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Default DHT bootstrap nodes
    pub fn defaults() -> Vec<Self> {
        vec![
            DhtBootstrapNode::new("router.bittorrent.com".to_string(), 6881),
            DhtBootstrapNode::new("dht.transmissionbt.com".to_string(), 6881),
            DhtBootstrapNode::new("router.utorrent.com".to_string(), 6881),
            DhtBootstrapNode::new("dht.libtorrent.org".to_string(), 25401),
        ]
    }
}

/// Tracker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerConfig {
    /// Custom tracker URLs to add
    pub additional_trackers: Vec<String>,
    /// Whether to use DHT for trackerless torrents
    pub enable_dht: bool,
    /// Whether to use PEX (Peer Exchange)
    pub enable_pex: bool,
    /// Whether to use LSD (Local Service Discovery)
    pub enable_lsd: bool,
    /// DHT bootstrap nodes
    pub bootstrap_nodes: Vec<DhtBootstrapNode>,
}

impl Default for TrackerConfig {
    fn default() -> Self {
        Self {
            additional_trackers: vec![],
            enable_dht: true,
            enable_pex: true,
            enable_lsd: true,
            bootstrap_nodes: DhtBootstrapNode::defaults(),
        }
    }
}

/// Advanced torrent options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedTorrentOptions {
    /// Web seed URLs for HTTP/HTTPS fallback
    pub web_seeds: Vec<WebSeed>,
    /// Protocol encryption settings
    pub encryption: EncryptionConfig,
    /// Tracker configuration
    pub tracker_config: TrackerConfig,
    /// Maximum connections per torrent
    pub max_connections: Option<usize>,
    /// Maximum upload slots
    pub max_upload_slots: Option<usize>,
    /// Seed ratio limit (stop seeding after ratio reached)
    pub seed_ratio_limit: Option<f64>,
    /// Seed time limit in seconds
    pub seed_time_limit: Option<u64>,
    /// Share ratio limit (0.0 = download only, 1.0 = 1:1 ratio)
    pub share_ratio_limit: Option<f64>,
}

impl Default for AdvancedTorrentOptions {
    fn default() -> Self {
        Self {
            web_seeds: vec![],
            encryption: EncryptionConfig::default(),
            tracker_config: TrackerConfig::default(),
            max_connections: Some(200),
            max_upload_slots: Some(50),
            seed_ratio_limit: Some(2.0),
            seed_time_limit: None,
            share_ratio_limit: Some(1.0),
        }
    }
}

/// Torrent piece selection strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PieceSelectionStrategy {
    /// Sequential - download pieces in order
    Sequential,
    /// Rarest first - download rarest pieces first
    RarestFirst,
    /// Random - random piece selection
    Random,
    /// EndGame - used near completion
    EndGame,
}

impl Default for PieceSelectionStrategy {
    fn default() -> Self {
        PieceSelectionStrategy::RarestFirst
    }
}

/// IP filter for blocking specific IPs or ranges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpFilter {
    /// Blocked IP addresses
    pub blocked_ips: Vec<String>,
    /// Blocked IP ranges (CIDR notation)
    pub blocked_ranges: Vec<String>,
    /// Whether to enable IP filtering
    pub enabled: bool,
}

impl Default for IpFilter {
    fn default() -> Self {
        Self {
            blocked_ips: vec![],
            blocked_ranges: vec![],
            enabled: false,
        }
    }
}

impl IpFilter {
    pub fn add_ip(&mut self, ip: String) {
        if !self.blocked_ips.contains(&ip) {
            self.blocked_ips.push(ip);
        }
    }

    pub fn add_range(&mut self, range: String) {
        if !self.blocked_ranges.contains(&range) {
            self.blocked_ranges.push(range);
        }
    }

    pub fn remove_ip(&mut self, ip: &str) {
        self.blocked_ips.retain(|i| i != ip);
    }

    pub fn is_blocked(&self, ip: &str) -> bool {
        if !self.enabled {
            return false;
        }
        
        // Simple exact match - production would need CIDR parsing
        self.blocked_ips.contains(&ip.to_string())
    }
}

/// Port forwarding configuration (UPnP/NAT-PMP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardingConfig {
    /// Enable UPnP
    pub enable_upnp: bool,
    /// Enable NAT-PMP
    pub enable_nat_pmp: bool,
    /// Port range to forward
    pub port_range: (u16, u16),
}

impl Default for PortForwardingConfig {
    fn default() -> Self {
        Self {
            enable_upnp: true,
            enable_nat_pmp: true,
            port_range: (6881, 6889),
        }
    }
}

/// Super seeding mode (BEP 16)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperSeedingConfig {
    pub enabled: bool,
}

impl Default for SuperSeedingConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Complete advanced configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentAdvancedConfig {
    pub options: AdvancedTorrentOptions,
    pub piece_selection: PieceSelectionStrategy,
    pub ip_filter: IpFilter,
    pub port_forwarding: PortForwardingConfig,
    pub super_seeding: SuperSeedingConfig,
}

impl Default for TorrentAdvancedConfig {
    fn default() -> Self {
        Self {
            options: AdvancedTorrentOptions::default(),
            piece_selection: PieceSelectionStrategy::default(),
            ip_filter: IpFilter::default(),
            port_forwarding: PortForwardingConfig::default(),
            super_seeding: SuperSeedingConfig::default(),
        }
    }
}

/// Web seed downloader helper
pub struct WebSeedDownloader {
    client: reqwest::Client,
}

impl WebSeedDownloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Download a piece from a web seed
    pub async fn download_piece(
        &self,
        web_seed: &WebSeed,
        file_path: &str,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, AppError> {
        let url = web_seed.file_url(file_path);
        
        let response = self.client
            .get(&url)
            .header("Range", format!("bytes={}-{}", offset, offset + length - 1))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Web seed request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "Web seed returned status: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await
            .map_err(|e| AppError::NetworkError(format!("Failed to read response: {}", e)))?;

        Ok(bytes.to_vec())
    }

    /// Check if web seed supports range requests
    pub async fn supports_range(&self, url: &str) -> Result<bool, AppError> {
        let response = self.client
            .head(url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("HEAD request failed: {}", e)))?;

        Ok(response.headers().contains_key("accept-ranges"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_seed_url_construction() {
        let web_seed = WebSeed::new(
            "http://example.com/downloads".to_string(),
            WebSeedType::GetRight,
        );

        assert_eq!(
            web_seed.file_url("file.txt"),
            "http://example.com/downloads/file.txt"
        );

        assert_eq!(
            web_seed.file_url("/folder/file.txt"),
            "http://example.com/downloads/folder/file.txt"
        );
    }

    #[test]
    fn test_encryption_config() {
        let disabled = EncryptionConfig::disabled();
        assert!(!disabled.enabled);
        assert_eq!(disabled.mode, EncryptionMode::Disabled);

        let required = EncryptionConfig::required();
        assert!(required.enabled);
        assert_eq!(required.mode, EncryptionMode::Required);
    }

    #[test]
    fn test_ip_filter() {
        let mut filter = IpFilter::default();
        filter.enabled = true;

        filter.add_ip("192.168.1.1".to_string());
        assert!(filter.is_blocked("192.168.1.1"));
        assert!(!filter.is_blocked("192.168.1.2"));

        filter.remove_ip("192.168.1.1");
        assert!(!filter.is_blocked("192.168.1.1"));
    }

    #[test]
    fn test_dht_bootstrap_defaults() {
        let nodes = DhtBootstrapNode::defaults();
        assert!(nodes.len() >= 3);
        assert!(nodes.iter().any(|n| n.host.contains("router.bittorrent.com")));
    }

    #[test]
    fn test_advanced_options_defaults() {
        let options = AdvancedTorrentOptions::default();
        assert!(options.encryption.enabled);
        assert!(options.tracker_config.enable_dht);
        assert!(options.tracker_config.enable_pex);
        assert_eq!(options.max_connections, Some(200));
        assert_eq!(options.seed_ratio_limit, Some(2.0));
    }
}
