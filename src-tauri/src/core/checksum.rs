// src-tauri/src/core/checksum.rs

use sha2::{Sha256, Digest as Sha2Digest};
use md5::Md5;
use tokio::io::AsyncReadExt;
use std::path::Path;
use crate::utils::error::DownloadError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChecksumAlgorithm {
    Md5,
    Sha256,
    Crc32,
}

impl ChecksumAlgorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "md5" => Some(Self::Md5),
            "sha256" | "sha-256" => Some(Self::Sha256),
            "crc32" => Some(Self::Crc32),
            _ => None,
        }
    }
}

impl std::fmt::Display for ChecksumAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChecksumAlgorithm::Md5 => "md5",
            ChecksumAlgorithm::Sha256 => "sha256",
            ChecksumAlgorithm::Crc32 => "crc32",
        };
        write!(f, "{}", s)
    }
}

pub struct ChecksumVerifier;

impl ChecksumVerifier {
    /// Calculate checksum of a file
    pub async fn calculate(
        file_path: &Path,
        algorithm: &ChecksumAlgorithm,
    ) -> Result<String, DownloadError> {
        tracing::info!(
            "Calculating {:?} checksum for: {}",
            algorithm,
            file_path.display()
        );

        let mut file = tokio::fs::File::open(file_path)
            .await
            .map_err(|e| DownloadError::FileError(
                format!("Cannot open file for checksum: {}", e)
            ))?;

        let checksum = match algorithm {
            ChecksumAlgorithm::Md5 => {
                Self::calculate_md5(&mut file).await?
            }
            ChecksumAlgorithm::Sha256 => {
                Self::calculate_sha256(&mut file).await?
            }
            ChecksumAlgorithm::Crc32 => {
                Self::calculate_crc32(&mut file).await?
            }
        };

        tracing::info!("Checksum: {}", checksum);
        Ok(checksum)
    }

    /// Verify file checksum against expected value
    pub async fn verify(
        file_path: &Path,
        expected: &str,
        algorithm: &ChecksumAlgorithm,
    ) -> Result<bool, DownloadError> {
        let actual = Self::calculate(file_path, algorithm).await?;
        let matches = actual.eq_ignore_ascii_case(expected);

        if !matches {
            tracing::warn!(
                "Checksum mismatch! Expected: {}, Got: {}",
                expected,
                actual
            );
        }

        Ok(matches)
    }

    async fn calculate_md5(
        file: &mut tokio::fs::File,
    ) -> Result<String, DownloadError> {
        let mut hasher = Md5::new();
        let mut buffer = vec![0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)
                .await
                .map_err(|e| DownloadError::FileError(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    async fn calculate_sha256(
        file: &mut tokio::fs::File,
    ) -> Result<String, DownloadError> {
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)
                .await
                .map_err(|e| DownloadError::FileError(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    async fn calculate_crc32(
        file: &mut tokio::fs::File,
    ) -> Result<String, DownloadError> {
        let mut hasher = crc32fast::Hasher::new();
        let mut buffer = vec![0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)
                .await
                .map_err(|e| DownloadError::FileError(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:08x}", hasher.finalize()))
    }
}