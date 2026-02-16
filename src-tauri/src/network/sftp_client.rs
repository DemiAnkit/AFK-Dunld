// src-tauri/src/network/sftp_client.rs

use std::path::PathBuf;
use std::net::TcpStream;
use ssh2::Session;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

use crate::utils::error::DownloadError;

/// SFTP client for secure file transfers over SSH
pub struct SftpClient {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    key_path: Option<PathBuf>,
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpFileInfo {
    pub file_name: String,
    pub file_size: Option<u64>,
    pub is_dir: bool,
    pub modified: Option<u64>,
}

impl SftpClient {
    /// Create a new SFTP client
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        key_path: Option<PathBuf>,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            key_path,
        }
    }

    /// Parse SFTP URL and create client
    /// Format: sftp://[user[:password]@]host[:port]/path
    pub fn from_url(url: &str, password: Option<String>, key_path: Option<PathBuf>) -> Result<(Self, String), DownloadError> {
        let parsed = url::Url::parse(url)
            .map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;

        let scheme = parsed.scheme();
        if scheme != "sftp" {
            return Err(DownloadError::InvalidUrl(
                format!("Unsupported scheme: {} (expected sftp)", scheme)
            ));
        }

        let host = parsed.host_str()
            .ok_or_else(|| DownloadError::InvalidUrl("Missing host".to_string()))?
            .to_string();

        let port = parsed.port().unwrap_or(22);

        let username = if parsed.username().is_empty() {
            "anonymous".to_string()
        } else {
            parsed.username().to_string()
        };

        // Password from URL takes precedence if provided
        let url_password = parsed.password().map(|s| s.to_string());
        let final_password = url_password.or(password);

        let path = parsed.path().to_string();

        Ok((
            Self::new(host, port, username, final_password, key_path),
            path,
        ))
    }

    /// Get file information from SFTP server
    pub async fn get_file_info(&self, remote_path: &str) -> Result<SftpFileInfo, DownloadError> {
        let session = self.connect()?;
        let sftp = session.sftp()
            .map_err(|e| DownloadError::NetworkError(format!("SFTP init failed: {}", e)))?;

        let stat = sftp.stat(std::path::Path::new(remote_path))
            .map_err(|e| DownloadError::NetworkError(format!("Failed to stat file: {}", e)))?;

        let file_name = remote_path
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string();

        Ok(SftpFileInfo {
            file_name,
            file_size: stat.size,
            is_dir: stat.is_dir(),
            modified: stat.mtime,
        })
    }

    /// List directory contents
    pub async fn list_directory(&self, remote_path: &str) -> Result<Vec<SftpFileInfo>, DownloadError> {
        let session = self.connect()?;
        let sftp = session.sftp()
            .map_err(|e| DownloadError::NetworkError(format!("SFTP init failed: {}", e)))?;

        let entries = sftp.readdir(std::path::Path::new(remote_path))
            .map_err(|e| DownloadError::NetworkError(format!("Failed to list directory: {}", e)))?;

        let mut results = Vec::new();
        for (path, stat) in entries {
            if let Some(file_name) = path.file_name() {
                results.push(SftpFileInfo {
                    file_name: file_name.to_string_lossy().to_string(),
                    file_size: stat.size,
                    is_dir: stat.is_dir(),
                    modified: stat.mtime,
                });
            }
        }

        Ok(results)
    }

    /// Download a file from SFTP server with resume support
    pub async fn download_file(
        &self,
        remote_path: &str,
        local_path: &PathBuf,
        resume_from: Option<u64>,
    ) -> Result<u64, DownloadError> {
        let session = self.connect()?;
        let sftp = session.sftp()
            .map_err(|e| DownloadError::NetworkError(format!("SFTP init failed: {}", e)))?;

        // Get file size
        let stat = sftp.stat(std::path::Path::new(remote_path))
            .map_err(|e| DownloadError::NetworkError(format!("Failed to stat file: {}", e)))?;

        let total_size = stat.size.unwrap_or(0);

        // Open remote file
        let mut remote_file = sftp.open(std::path::Path::new(remote_path))
            .map_err(|e| DownloadError::NetworkError(format!("Failed to open remote file: {}", e)))?;

        // Open local file for writing (append if resuming)
        let mut local_file = if let Some(offset) = resume_from {
            info!("Resuming SFTP download from byte {}", offset);
            
            // Seek remote file to resume position
            use std::io::Seek;
            remote_file.seek(std::io::SeekFrom::Start(offset))
                .map_err(|e| DownloadError::NetworkError(format!("Failed to seek: {}", e)))?;
            
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

        // Read from remote and write to local
        let mut total_bytes = resume_from.unwrap_or(0);
        let mut buffer = vec![0u8; 32768]; // 32KB buffer

        loop {
            use std::io::Read;
            match remote_file.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    local_file.write_all(&buffer[..n])
                        .await
                        .map_err(|e| DownloadError::FileError(format!("Write error: {}", e)))?;
                    total_bytes += n as u64;
                }
                Err(e) => {
                    return Err(DownloadError::NetworkError(format!("Read error: {}", e)));
                }
            }
        }

        local_file.flush().await
            .map_err(|e| DownloadError::FileError(format!("Flush error: {}", e)))?;

        info!("SFTP download completed: {} bytes", total_bytes);
        Ok(total_bytes)
    }

    /// Upload a file to SFTP server
    pub async fn upload_file(
        &self,
        local_path: &PathBuf,
        remote_path: &str,
    ) -> Result<u64, DownloadError> {
        let session = self.connect()?;
        let sftp = session.sftp()
            .map_err(|e| DownloadError::NetworkError(format!("SFTP init failed: {}", e)))?;

        // Open local file for reading
        let mut local_file = tokio::fs::File::open(local_path)
            .await
            .map_err(|e| DownloadError::FileError(format!("Cannot open local file: {}", e)))?;

        // Create remote file
        let mut remote_file = sftp.create(std::path::Path::new(remote_path))
            .map_err(|e| DownloadError::NetworkError(format!("Failed to create remote file: {}", e)))?;

        // Read from local and write to remote
        let mut total_bytes = 0u64;
        let mut buffer = vec![0u8; 32768]; // 32KB buffer

        loop {
            use tokio::io::AsyncReadExt;
            match local_file.read(&mut buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    use std::io::Write;
                    remote_file.write_all(&buffer[..n])
                        .map_err(|e| DownloadError::NetworkError(format!("Write error: {}", e)))?;
                    total_bytes += n as u64;
                }
                Err(e) => {
                    return Err(DownloadError::FileError(format!("Read error: {}", e)));
                }
            }
        }

        info!("SFTP upload completed: {} bytes", total_bytes);
        Ok(total_bytes)
    }

    /// Connect to SFTP server and authenticate
    fn connect(&self) -> Result<Session, DownloadError> {
        let addr = format!("{}:{}", self.host, self.port);
        
        debug!("Connecting to SFTP server: {}", addr);
        
        let tcp = TcpStream::connect(&addr)
            .map_err(|e| DownloadError::NetworkError(format!("TCP connection failed: {}", e)))?;

        let mut session = Session::new()
            .map_err(|e| DownloadError::NetworkError(format!("Session creation failed: {}", e)))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| DownloadError::NetworkError(format!("SSH handshake failed: {}", e)))?;

        // Authenticate
        if let Some(key_path) = &self.key_path {
            // Public key authentication
            debug!("Authenticating with public key");
            session.userauth_pubkey_file(
                &self.username,
                None,
                key_path,
                self.password.as_deref(),
            ).map_err(|e| DownloadError::AuthenticationFailed(format!("Public key auth failed: {}", e)))?;
        } else if let Some(password) = &self.password {
            // Password authentication
            debug!("Authenticating with password");
            session.userauth_password(&self.username, password)
                .map_err(|e| DownloadError::AuthenticationFailed(format!("Password auth failed: {}", e)))?;
        } else {
            return Err(DownloadError::AuthenticationFailed(
                "No authentication method provided (password or key required)".to_string()
            ));
        }

        if !session.authenticated() {
            return Err(DownloadError::AuthenticationFailed("Authentication failed".to_string()));
        }

        debug!("SFTP authentication successful");
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sftp_url() {
        let (client, path) = SftpClient::from_url(
            "sftp://user@example.com:2222/path/to/file.zip",
            Some("password".to_string()),
            None
        ).unwrap();
        
        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 2222);
        assert_eq!(client.username, "user");
        assert_eq!(client.password, Some("password".to_string()));
        assert_eq!(path, "/path/to/file.zip");
    }

    #[test]
    fn test_parse_sftp_url_with_password_in_url() {
        let (client, path) = SftpClient::from_url(
            "sftp://user:urlpass@example.com/file.zip",
            Some("otherpass".to_string()),
            None
        ).unwrap();
        
        // URL password should take precedence
        assert_eq!(client.password, Some("urlpass".to_string()));
    }

    #[test]
    fn test_parse_sftp_default_port() {
        let (client, _) = SftpClient::from_url(
            "sftp://example.com/file.zip",
            None,
            None
        ).unwrap();
        
        assert_eq!(client.port, 22);
    }
}
