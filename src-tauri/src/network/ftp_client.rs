// src-tauri/src/network/ftp_client.rs

use async_native_tls::TlsConnector;
use std::path::PathBuf;
use suppaftp::{AsyncFtpStream, AsyncNativeTlsFtpStream};
use suppaftp::types::FileType;
use tokio::io::AsyncWriteExt;
use futures::io::AsyncReadExt;
use tracing::{debug, info};

use crate::utils::error::DownloadError;

/// FTP client for downloading files via FTP/FTPS
pub struct FtpClient {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    use_tls: bool,
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpFileInfo {
    pub file_name: String,
    pub file_size: Option<u64>,
}

impl FtpClient {
    /// Create a new FTP client from a parsed URL
    pub fn new(
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
        use_tls: bool,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            use_tls,
        }
    }

    /// Parse FTP URL and create client
    /// Format: ftp://[user[:password]@]host[:port]/path
    ///         ftps://[user[:password]@]host[:port]/path
    pub fn from_url(url: &str) -> Result<(Self, String), DownloadError> {
        let parsed = url::Url::parse(url)
            .map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;

        let scheme = parsed.scheme();
        let use_tls = match scheme {
            "ftp" => false,
            "ftps" => true,
            _ => return Err(DownloadError::InvalidUrl(
                format!("Unsupported scheme: {}", scheme)
            )),
        };

        let host = parsed.host_str()
            .ok_or_else(|| DownloadError::InvalidUrl("Missing host".to_string()))?
            .to_string();

        let port = parsed.port().unwrap_or(21);

        let username = if parsed.username().is_empty() {
            None
        } else {
            Some(parsed.username().to_string())
        };

        let password = parsed.password().map(|s| s.to_string());

        let path = parsed.path().to_string();

        Ok((
            Self::new(host, port, username, password, use_tls),
            path,
        ))
    }

    /// Get file information from FTP server
    pub async fn get_file_info(&self, remote_path: &str) -> Result<FtpFileInfo, DownloadError> {
        if self.use_tls {
            self.get_file_info_tls(remote_path).await
        } else {
            self.get_file_info_plain(remote_path).await
        }
    }

    async fn get_file_info_plain(&self, remote_path: &str) -> Result<FtpFileInfo, DownloadError> {
        let mut ftp = self.connect_plain().await?;
        
        // Get file size
        let size = ftp.size(remote_path).await.ok();
        
        // Extract filename from path
        let file_name = remote_path
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string();

        let _ = ftp.quit().await;

        Ok(FtpFileInfo {
            file_name,
            file_size: size.map(|s| s as u64),
        })
    }

    async fn get_file_info_tls(&self, remote_path: &str) -> Result<FtpFileInfo, DownloadError> {
        let mut ftp = self.connect_tls().await?;
        
        // Get file size
        let size = ftp.size(remote_path).await.ok();
        
        // Extract filename from path
        let file_name = remote_path
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string();

        let _ = ftp.quit().await;

        Ok(FtpFileInfo {
            file_name,
            file_size: size.map(|s| s as u64),
        })
    }

    /// Download a file from FTP server
    /// Returns the number of bytes downloaded
    pub async fn download_file(
        &self,
        remote_path: &str,
        local_path: &PathBuf,
        resume_from: Option<u64>,
    ) -> Result<u64, DownloadError> {
        if self.use_tls {
            self.download_file_tls(remote_path, local_path, resume_from).await
        } else {
            self.download_file_plain(remote_path, local_path, resume_from).await
        }
    }

    async fn download_file_plain(
        &self,
        remote_path: &str,
        local_path: &PathBuf,
        resume_from: Option<u64>,
    ) -> Result<u64, DownloadError> {
        let mut ftp = self.connect_plain().await?;

        // Set binary mode
        ftp.transfer_type(FileType::Binary)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("Failed to set binary mode: {}", e)))?;

        // Open local file for writing (append if resuming)
        let mut file = if let Some(offset) = resume_from {
            info!("Resuming FTP download from byte {}", offset);
            
            // Resume transfer
            ftp.resume_transfer(offset as usize)
                .await
                .map_err(|e| DownloadError::NetworkError(format!("Failed to resume transfer: {}", e)))?;
            
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(local_path)
                .await
                .map_err(|e| DownloadError::FileError(format!("Cannot open file for resume: {}", e)))?
        } else {
            tokio::fs::File::create(local_path)
                .await
                .map_err(|e| DownloadError::FileError(format!("Cannot create file: {}", e)))?
        };

        // Retrieve file
        let mut stream = ftp.retr_as_stream(remote_path)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("Failed to retrieve file: {}", e)))?;

        // Read from stream and write to file
        let mut total_bytes = resume_from.unwrap_or(0);
        let mut buffer = vec![0u8; 8192];

        loop {
            match futures::io::AsyncReadExt::read(&mut stream, &mut buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    file.write_all(&buffer[..n])
                        .await
                        .map_err(|e| DownloadError::FileError(format!("Write error: {}", e)))?;
                    total_bytes += n as u64;
                }
                Err(e) => {
                    return Err(DownloadError::NetworkError(format!("Read error: {}", e)));
                }
            }
        }

        file.flush().await
            .map_err(|e| DownloadError::FileError(format!("Flush error: {}", e)))?;

        // Finalize transfer
        let _ = ftp.finalize_retr_stream(stream).await;
        let _ = ftp.quit().await;

        info!("FTP download completed: {} bytes", total_bytes);
        Ok(total_bytes)
    }

    async fn download_file_tls(
        &self,
        remote_path: &str,
        local_path: &PathBuf,
        resume_from: Option<u64>,
    ) -> Result<u64, DownloadError> {
        let mut ftp = self.connect_tls().await?;

        // Set binary mode
        ftp.transfer_type(FileType::Binary)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("Failed to set binary mode: {}", e)))?;

        // Open local file for writing (append if resuming)
        let mut file = if let Some(offset) = resume_from {
            info!("Resuming FTPS download from byte {}", offset);
            
            // Resume transfer
            ftp.resume_transfer(offset as usize)
                .await
                .map_err(|e| DownloadError::NetworkError(format!("Failed to resume transfer: {}", e)))?;
            
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(local_path)
                .await
                .map_err(|e| DownloadError::FileError(format!("Cannot open file for resume: {}", e)))?
        } else {
            tokio::fs::File::create(local_path)
                .await
                .map_err(|e| DownloadError::FileError(format!("Cannot create file: {}", e)))?
        };

        // Retrieve file
        let mut stream = ftp.retr_as_stream(remote_path)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("Failed to retrieve file: {}", e)))?;

        // Read from stream and write to file
        let mut total_bytes = resume_from.unwrap_or(0);
        let mut buffer = vec![0u8; 8192];

        loop {
            match futures::io::AsyncReadExt::read(&mut stream, &mut buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    file.write_all(&buffer[..n])
                        .await
                        .map_err(|e| DownloadError::FileError(format!("Write error: {}", e)))?;
                    total_bytes += n as u64;
                }
                Err(e) => {
                    return Err(DownloadError::NetworkError(format!("Read error: {}", e)));
                }
            }
        }

        file.flush().await
            .map_err(|e| DownloadError::FileError(format!("Flush error: {}", e)))?;

        // Finalize transfer
        let _ = ftp.finalize_retr_stream(stream).await;
        let _ = ftp.quit().await;

        info!("FTPS download completed: {} bytes", total_bytes);
        Ok(total_bytes)
    }

    /// Connect to FTP server (plain)
    async fn connect_plain(&self) -> Result<AsyncFtpStream, DownloadError> {
        let addr = format!("{}:{}", self.host, self.port);
        
        debug!("Connecting to FTP server: {}", addr);
        
        let mut ftp = AsyncFtpStream::connect(&addr)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("FTP connection failed: {}", e)))?;

        // Login
        let username = self.username.as_deref().unwrap_or("anonymous");
        let password = self.password.as_deref().unwrap_or("anonymous@");
        
        ftp.login(username, password)
            .await
            .map_err(|e| DownloadError::AuthenticationFailed(format!("FTP login failed: {}", e)))?;

        debug!("FTP login successful");
        Ok(ftp)
    }

    /// Connect to FTP server (TLS)
    async fn connect_tls(&self) -> Result<AsyncNativeTlsFtpStream, DownloadError> {
        let addr = format!("{}:{}", self.host, self.port);
        
        debug!("Connecting to FTPS server: {}", addr);
        
        // Connect with TLS directly
        let mut ftp = AsyncNativeTlsFtpStream::connect(&addr)
            .await
            .map_err(|e| DownloadError::NetworkError(format!("FTPS connection failed: {}", e)))?;

        // Login
        let username = self.username.as_deref().unwrap_or("anonymous");
        let password = self.password.as_deref().unwrap_or("anonymous@");
        
        ftp.login(username, password)
            .await
            .map_err(|e| DownloadError::AuthenticationFailed(format!("FTPS login failed: {}", e)))?;

        debug!("FTPS login successful");
        Ok(ftp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ftp_url() {
        let (client, path) = FtpClient::from_url("ftp://user:pass@example.com:2121/path/to/file.zip")
            .unwrap();
        
        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 2121);
        assert_eq!(client.username, Some("user".to_string()));
        assert_eq!(client.password, Some("pass".to_string()));
        assert_eq!(path, "/path/to/file.zip");
        assert!(!client.use_tls);
    }

    #[test]
    fn test_parse_ftps_url() {
        let (client, path) = FtpClient::from_url("ftps://example.com/file.zip")
            .unwrap();
        
        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 21);
        assert_eq!(client.username, None);
        assert_eq!(client.password, None);
        assert_eq!(path, "/file.zip");
        assert!(client.use_tls);
    }

    #[test]
    fn test_parse_anonymous_ftp() {
        let (client, path) = FtpClient::from_url("ftp://ftp.example.com/pub/file.tar.gz")
            .unwrap();
        
        assert_eq!(client.host, "ftp.example.com");
        assert_eq!(client.username, None);
        assert_eq!(path, "/pub/file.tar.gz");
    }
}
