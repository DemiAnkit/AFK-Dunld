// src-tauri/src/network/bencode_parser.rs
// Bencode parsing utilities for torrent files

use serde::{Deserialize, Serialize};
use serde_bencode;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::utils::error::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentFile {
    pub info: TorrentInfo,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(rename = "announce-list")]
    #[serde(default)]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(rename = "created by")]
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(rename = "creation date")]
    #[serde(default)]
    pub creation_date: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TorrentInfo {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: serde_bencode::value::ByteString,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<FileInfo>>,
    #[serde(default)]
    pub private: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileInfo {
    pub length: i64,
    pub path: Vec<String>,
}

impl TorrentFile {
    /// Parse a torrent file from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, AppError> {
        serde_bencode::from_bytes::<TorrentFile>(data)
            .map_err(|e| AppError::TorrentError(format!("Failed to parse torrent file: {}", e)))
    }

    /// Parse a torrent file from a path
    pub async fn from_file(path: &PathBuf) -> Result<Self, AppError> {
        let data = tokio::fs::read(path).await
            .map_err(|e| AppError::TorrentError(format!("Failed to read torrent file: {}", e)))?;
        Self::from_bytes(&data)
    }

    /// Calculate the info hash (SHA1 hash of the bencoded info dictionary)
    pub fn info_hash(&self) -> Result<String, AppError> {
        let info_bytes = serde_bencode::to_bytes(&self.info)
            .map_err(|e| AppError::TorrentError(format!("Failed to encode info dict: {}", e)))?;
        
        let mut hasher = Sha1::new();
        hasher.update(&info_bytes);
        let hash = hasher.finalize();
        
        Ok(hex::encode(hash))
    }

    /// Get total size of the torrent
    pub fn total_size(&self) -> u64 {
        if let Some(length) = self.info.length {
            length as u64
        } else if let Some(ref files) = self.info.files {
            files.iter().map(|f| f.length as u64).sum()
        } else {
            0
        }
    }

    /// Get number of pieces
    pub fn num_pieces(&self) -> u64 {
        (self.info.pieces.len() / 20) as u64
    }

    /// Get list of files in the torrent
    pub fn file_list(&self) -> Vec<(PathBuf, u64)> {
        if let Some(ref files) = self.info.files {
            files.iter().map(|f| {
                let path: PathBuf = f.path.iter().collect();
                (path, f.length as u64)
            }).collect()
        } else {
            vec![(PathBuf::from(&self.info.name), self.info.length.unwrap_or(0) as u64)]
        }
    }

    /// Get all tracker URLs
    pub fn trackers(&self) -> Vec<String> {
        let mut trackers = Vec::new();
        
        if let Some(ref announce) = self.announce {
            trackers.push(announce.clone());
        }
        
        if let Some(ref announce_list) = self.announce_list {
            for tier in announce_list {
                for tracker in tier {
                    if !trackers.contains(tracker) {
                        trackers.push(tracker.clone());
                    }
                }
            }
        }
        
        trackers
    }

    /// Check if torrent is private
    pub fn is_private(&self) -> bool {
        self.info.private.unwrap_or(0) == 1
    }
}

/// Parse a magnet link into its components
#[derive(Debug, Clone)]
pub struct MagnetLink {
    pub info_hash: String,
    pub display_name: Option<String>,
    pub trackers: Vec<String>,
    pub exact_length: Option<u64>,
}

impl MagnetLink {
    /// Parse a magnet URI
    pub fn parse(magnet_uri: &str) -> Result<Self, AppError> {
        if !magnet_uri.starts_with("magnet:?") {
            return Err(AppError::TorrentError("Invalid magnet link".to_string()));
        }

        let params: HashMap<String, String> = magnet_uri[8..]
            .split('&')
            .filter_map(|param| {
                let mut parts = param.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();

        let info_hash = params.get("xt")
            .and_then(|xt| xt.strip_prefix("urn:btih:"))
            .ok_or_else(|| AppError::TorrentError("Missing info hash in magnet link".to_string()))?
            .to_string();

        let display_name = params.get("dn")
            .map(|dn| urlencoding::decode(dn).unwrap_or_default().to_string());

        let trackers = params.iter()
            .filter(|(k, _)| k == &"tr")
            .filter_map(|(_, v)| urlencoding::decode(v).ok().map(|s| s.to_string()))
            .collect();

        let exact_length = params.get("xl")
            .and_then(|xl| xl.parse::<u64>().ok());

        Ok(MagnetLink {
            info_hash,
            display_name,
            trackers,
            exact_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnet_parsing() {
        let magnet = "magnet:?xt=urn:btih:1234567890abcdef1234567890abcdef12345678&dn=test%20file&tr=http://tracker.example.com";
        let parsed = MagnetLink::parse(magnet).unwrap();
        
        assert_eq!(parsed.info_hash, "1234567890abcdef1234567890abcdef12345678");
        assert_eq!(parsed.display_name, Some("test file".to_string()));
        assert_eq!(parsed.trackers.len(), 1);
    }
}
