// src-tauri/src/network/http_client.rs

use reqwest::{Client, Response, header};
use std::time::Duration;
use crate::utils::constants::*;
use crate::utils::error::DownloadError;
use crate::network::url_parser::UrlParser;

/// Information about a remote file
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteFileInfo {
    pub url: String,
    pub file_name: String,
    pub total_size: Option<u64>,
    pub supports_range: bool,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub redirect_url: Option<String>,
}

/// HTTP client wrapper with retry and proxy support
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(
        proxy_url: Option<&str>,
        connect_timeout: u64,
        read_timeout: u64,
    ) -> Result<Self, DownloadError> {
        let mut builder = Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(connect_timeout))
            .timeout(Duration::from_secs(read_timeout))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .redirect(reqwest::redirect::Policy::limited(10))
            .gzip(true)
            .brotli(true)
            .deflate(true);

        // Configure proxy
        if let Some(proxy_str) = proxy_url {
            if !proxy_str.is_empty() {
                let proxy = reqwest::Proxy::all(proxy_str)
                    .map_err(|e| DownloadError::NetworkError(
                        format!("Invalid proxy: {}", e)
                    ))?;
                builder = builder.proxy(proxy);
                tracing::info!("Using proxy: {}", proxy_str);
            }
        }

        let client = builder.build()
            .map_err(|e| DownloadError::NetworkError(
                format!("Failed to build HTTP client: {}", e)
            ))?;

        Ok(Self { client })
    }

    /// Get file information using HEAD request
    pub async fn get_file_info(
        &self,
        url: &str,
    ) -> Result<RemoteFileInfo, DownloadError> {
        tracing::debug!("Fetching file info: {}", url);

        // First try HEAD request
        let response = self.client
            .head(url)
            .send()
            .await
            .map_err(|e| DownloadError::NetworkError(e.to_string()))?;

        // Check for HTTP errors
        if !response.status().is_success() && !response.status().is_redirection() {
            return Err(DownloadError::ServerError {
                status: response.status().as_u16(),
                message: response.status().to_string(),
            });
        }

        let headers = response.headers().clone();
        let final_url = response.url().to_string();

        // Extract file size
        let total_size = headers
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        // Check range support
        let supports_range = headers
            .get(header::ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("bytes"))
            .unwrap_or(false);

        // Extract filename
        let file_name = headers
            .get(header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| UrlParser::extract_filename_from_header(v))
            .unwrap_or_else(|| {
                UrlParser::parse(&final_url)
                    .map(|p| p.filename)
                    .unwrap_or_else(|_| "download".to_string())
            });

        // Content type
        let content_type = headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        // ETag for resume verification
        let etag = headers
            .get(header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        // Last-Modified
        let last_modified = headers
            .get(header::LAST_MODIFIED)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let redirect_url = if final_url != url {
            Some(final_url)
        } else {
            None
        };

        let info = RemoteFileInfo {
            url: redirect_url.clone().unwrap_or_else(|| url.to_string()),
            file_name,
            total_size,
            supports_range,
            content_type,
            etag,
            last_modified,
            redirect_url,
        };

        tracing::info!(
            "File info: name={}, size={:?}, range_support={}, type={:?}",
            info.file_name,
            info.total_size,
            info.supports_range,
            info.content_type,
        );

        Ok(info)
    }

    /// Start a GET request with optional range header
    pub async fn get_range(
        &self,
        url: &str,
        start: u64,
        end: u64,
    ) -> Result<Response, DownloadError> {
        let range = format!("bytes={}-{}", start, end);
        tracing::debug!("GET {} Range: {}", url, range);

        let response = self.client
            .get(url)
            .header(header::RANGE, range)
            .send()
            .await
            .map_err(|e| DownloadError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DownloadError::ServerError {
                status: response.status().as_u16(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }

    /// Start a GET request for full file (no range)
    pub async fn get_full(
        &self,
        url: &str,
    ) -> Result<Response, DownloadError> {
        tracing::debug!("GET (full) {}", url);

        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| DownloadError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DownloadError::ServerError {
                status: response.status().as_u16(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }

    /// Start a GET request with resume from byte offset
    pub async fn get_resume(
        &self,
        url: &str,
        from_byte: u64,
    ) -> Result<Response, DownloadError> {
        let range = format!("bytes={}-", from_byte);
        tracing::debug!("GET (resume) {} Range: {}", url, range);

        let response = self.client
            .get(url)
            .header(header::RANGE, range)
            .send()
            .await
            .map_err(|e| DownloadError::NetworkError(e.to_string()))?;

        // 206 Partial Content = resume successful
        // 200 OK = server doesn't support resume, sending full file
        if response.status() != reqwest::StatusCode::PARTIAL_CONTENT
            && !response.status().is_success()
        {
            return Err(DownloadError::ServerError {
                status: response.status().as_u16(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = HttpClient::new(None, 30, 60);
        assert!(client.is_ok());
    }
}