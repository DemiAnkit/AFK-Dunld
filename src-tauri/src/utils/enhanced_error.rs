// Enhanced Error Handling with user-friendly messages
use serde::{Serialize, Deserialize};
use std::fmt;

/// User-friendly error representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserError {
    pub title: String,
    pub message: String,
    pub details: Option<String>,
    pub recovery_hint: Option<String>,
    pub error_code: String,
    pub retryable: bool,
}

impl UserError {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        error_code: impl Into<String>,
        retryable: bool,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            details: None,
            recovery_hint: None,
            error_code: error_code.into(),
            retryable,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn with_recovery_hint(mut self, hint: impl Into<String>) -> Self {
        self.recovery_hint = Some(hint.into());
        self
    }

    /// Convert from DownloadError to UserError
    pub fn from_download_error(error: &crate::utils::error::DownloadError) -> Self {
        use crate::utils::error::DownloadError;

        match error {
            DownloadError::NetworkError(msg) => {
                UserError::new(
                    "Network Error",
                    "Unable to connect to the server",
                    "NET_ERROR",
                    true,
                )
                .with_details(msg)
                .with_recovery_hint("Check your internet connection and try again")
            }

            DownloadError::FileError(msg) => {
                UserError::new(
                    "File Error",
                    "Problem accessing the file",
                    "FILE_ERROR",
                    true,
                )
                .with_details(msg)
                .with_recovery_hint("Check disk space and file permissions")
            }

            DownloadError::AuthenticationFailed(msg) => {
                UserError::new(
                    "Authentication Failed",
                    "Could not authenticate with the server",
                    "AUTH_ERROR",
                    false,
                )
                .with_details(msg)
                .with_recovery_hint("Check your username and password")
            }

            DownloadError::InvalidUrl(msg) => {
                UserError::new(
                    "Invalid URL",
                    "The download URL is not valid",
                    "INVALID_URL",
                    false,
                )
                .with_details(msg)
                .with_recovery_hint("Verify the URL format and try again")
            }

            DownloadError::Timeout { seconds } => {
                UserError::new(
                    "Connection Timeout",
                    &format!("The server took too long to respond ({}s)", seconds),
                    "TIMEOUT",
                    true,
                )
                .with_recovery_hint("Try again or check if the server is responding")
            }

            DownloadError::NotFound(msg) => {
                UserError::new(
                    "Not Found",
                    "The requested resource was not found",
                    "NOT_FOUND",
                    false,
                )
                .with_details(msg)
                .with_recovery_hint("Check if the file still exists on the server")
            }

            DownloadError::PermissionDenied => {
                UserError::new(
                    "Permission Denied",
                    "You don't have permission to access this resource",
                    "PERMISSION_DENIED",
                    false,
                )
                .with_recovery_hint("Contact the server administrator or check your credentials")
            }

            DownloadError::DiskFull => {
                UserError::new(
                    "Disk Full",
                    "Not enough space on disk to complete the download",
                    "DISK_FULL",
                    false,
                )
                .with_recovery_hint("Free up disk space and try again")
            }

            DownloadError::TorrentError(msg) => {
                UserError::new(
                    "Torrent Error",
                    "Problem with torrent download",
                    "TORRENT_ERROR",
                    true,
                )
                .with_details(msg)
                .with_recovery_hint("Check if the torrent is still active and has seeders")
            }

            DownloadError::Unknown(msg) => {
                UserError::new(
                    "Unknown Error",
                    "An unexpected error occurred",
                    "UNKNOWN",
                    true,
                )
                .with_details(msg)
                .with_recovery_hint("Try again or contact support if the problem persists")
            }

            _ => {
                UserError::new(
                    "Error",
                    "An error occurred during the operation",
                    "GENERAL_ERROR",
                    true,
                )
                .with_details(format!("{:?}", error))
            }
        }
    }

    /// Convert from AppError to UserError
    pub fn from_app_error(error: &crate::utils::error::AppError) -> Self {
        use crate::utils::error::AppError;

        match error {
            AppError::InvalidInput(msg) => {
                UserError::new(
                    "Invalid Input",
                    "The provided input is not valid",
                    "INVALID_INPUT",
                    false,
                )
                .with_details(msg)
                .with_recovery_hint("Check your input and try again")
            }

            AppError::NotFound(msg) => {
                UserError::new(
                    "Not Found",
                    "The requested item was not found",
                    "NOT_FOUND",
                    false,
                )
                .with_details(msg)
            }

            AppError::TorrentError(msg) => {
                UserError::new(
                    "Torrent Error",
                    "Problem with torrent operation",
                    "TORRENT_ERROR",
                    true,
                )
                .with_details(msg)
                .with_recovery_hint("Verify the magnet link or torrent file")
            }

            AppError::NotImplemented(msg) => {
                UserError::new(
                    "Not Implemented",
                    "This feature is not yet available",
                    "NOT_IMPLEMENTED",
                    false,
                )
                .with_details(msg)
            }

            _ => {
                UserError::new(
                    "Application Error",
                    "An application error occurred",
                    "APP_ERROR",
                    true,
                )
                .with_details(format!("{:?}", error))
            }
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.message)
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Calculate delay for a given attempt (exponential backoff)
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        let delay = self.initial_delay_ms as f64 * self.backoff_multiplier.powi(attempt as i32);
        delay.min(self.max_delay_ms as f64) as u64
    }
}

/// Retry a fallible operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    config: RetryConfig,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;

    for attempt in 0..config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                tracing::warn!(
                    "Attempt {}/{} failed: {}",
                    attempt + 1,
                    config.max_attempts,
                    error
                );

                last_error = Some(error);

                if attempt + 1 < config.max_attempts {
                    let delay = config.delay_for_attempt(attempt);
                    tracing::info!("Retrying in {}ms...", delay);
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_error_creation() {
        let error = UserError::new("Test Error", "Test message", "TEST_001", true)
            .with_details("Additional details")
            .with_recovery_hint("Try this to fix");

        assert_eq!(error.title, "Test Error");
        assert_eq!(error.message, "Test message");
        assert_eq!(error.error_code, "TEST_001");
        assert!(error.retryable);
        assert_eq!(error.details, Some("Additional details".to_string()));
        assert_eq!(error.recovery_hint, Some("Try this to fix".to_string()));
    }

    #[test]
    fn test_retry_config_delays() {
        let config = RetryConfig::default();

        assert_eq!(config.delay_for_attempt(0), 1000);
        assert_eq!(config.delay_for_attempt(1), 2000);
        assert_eq!(config.delay_for_attempt(2), 4000);
        assert_eq!(config.delay_for_attempt(3), 8000);
        
        // Should cap at max_delay_ms
        assert_eq!(config.delay_for_attempt(10), 30000);
    }

    #[tokio::test]
    async fn test_retry_succeeds_eventually() {
        let mut attempts = 0;
        let operation = || async {
            attempts += 1;
            if attempts < 3 {
                Err("Failed")
            } else {
                Ok("Success")
            }
        };

        let config = RetryConfig {
            max_attempts: 5,
            initial_delay_ms: 10,
            max_delay_ms: 100,
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(operation, config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_retry_fails_after_max_attempts() {
        let operation = || async { Err::<(), _>("Always fails") };

        let config = RetryConfig {
            max_attempts: 3,
            initial_delay_ms: 10,
            max_delay_ms: 100,
            backoff_multiplier: 2.0,
        };

        let result = retry_with_backoff(operation, config).await;
        assert!(result.is_err());
    }
}
