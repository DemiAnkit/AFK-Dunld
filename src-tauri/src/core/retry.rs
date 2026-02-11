// src-tauri/src/core/retry.rs

use std::time::Duration;
use tokio::time::sleep;
use crate::utils::constants::*;
use crate::utils::error::DownloadError;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial delay in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum delay in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Whether to add jitter to delays
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            initial_delay_ms: DEFAULT_RETRY_DELAY_MS,
            max_delay_ms: MAX_RETRY_DELAY_MS,
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Retry handler with exponential backoff
pub struct RetryHandler {
    config: RetryConfig,
}

impl RetryHandler {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Execute an async operation with retry logic
    pub async fn execute<F, Fut, T>(
        &self,
        operation_name: &str,
        mut operation: F,
    ) -> Result<T, DownloadError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, DownloadError>>,
    {
        let mut attempt = 0u32;

        loop {
            attempt += 1;
            tracing::debug!(
                "{}: attempt {}/{}",
                operation_name,
                attempt,
                self.config.max_retries + 1
            );

            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        tracing::info!(
                            "{}: succeeded on attempt {}",
                            operation_name,
                            attempt
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    // Don't retry on certain errors
                    if Self::is_non_retryable(&e) {
                        tracing::warn!(
                            "{}: non-retryable error: {}",
                            operation_name,
                            e
                        );
                        return Err(e);
                    }

                    if attempt > self.config.max_retries {
                        tracing::error!(
                            "{}: max retries exceeded ({} attempts)",
                            operation_name,
                            attempt
                        );
                        return Err(DownloadError::MaxRetriesExceeded {
                            retries: attempt,
                        });
                    }

                    let delay = self.calculate_delay(attempt);
                    tracing::warn!(
                        "{}: attempt {} failed ({}), retrying in {}ms",
                        operation_name,
                        attempt,
                        e,
                        delay.as_millis()
                    );

                    sleep(delay).await;
                }
            }
        }
    }

    /// Calculate delay with exponential backoff and optional jitter
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.config.initial_delay_ms as f64
            * self.config.backoff_multiplier.powi(attempt as i32 - 1);

        let delay_ms = base_delay.min(self.config.max_delay_ms as f64);

        let final_delay = if self.config.jitter {
            // Add random jitter: 50% to 150% of calculated delay
            let jitter_factor = 0.5 + rand_simple() * 1.0;
            (delay_ms * jitter_factor) as u64
        } else {
            delay_ms as u64
        };

        Duration::from_millis(final_delay)
    }

    /// Check if an error should not be retried
    fn is_non_retryable(error: &DownloadError) -> bool {
        matches!(
            error,
            DownloadError::Cancelled
                | DownloadError::Paused
                | DownloadError::ChecksumMismatch { .. }
                | DownloadError::InvalidUrl(_)
                | DownloadError::FileExists(_)
                | DownloadError::InsufficientDiskSpace
                | DownloadError::ServerError { status: 401, .. }
                | DownloadError::ServerError { status: 403, .. }
                | DownloadError::ServerError { status: 404, .. }
                | DownloadError::ServerError { status: 410, .. }
        )
    }
}

/// Simple pseudo-random number between 0.0 and 1.0
/// (avoiding external dependency for a simple use case)
fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}