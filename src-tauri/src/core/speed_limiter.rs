// src-tauri/src/core/speed_limiter.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration, Instant};

/// Rate limiter using token bucket algorithm
pub struct SpeedLimiter {
    /// Max bytes per second (None = unlimited)
    limit: Arc<RwLock<Option<u64>>>,
    /// Bytes consumed in current window
    bytes_in_window: Arc<RwLock<u64>>,
    /// Window start time
    window_start: Arc<RwLock<Instant>>,
    /// Window duration
    window_duration: Duration,
}

impl SpeedLimiter {
    pub fn new(limit: Option<u64>) -> Self {
        Self {
            limit: Arc::new(RwLock::new(limit)),
            bytes_in_window: Arc::new(RwLock::new(0)),
            window_start: Arc::new(RwLock::new(Instant::now())),
            window_duration: Duration::from_millis(100), // 100ms windows
        }
    }

    /// Throttle based on how many bytes are being written
    /// This should be called AFTER writing the bytes
    pub async fn throttle(&self, bytes: usize) {
        let limit = *self.limit.read().await;
        let limit = match limit {
            Some(l) if l > 0 => l,
            _ => return, // No limit
        };

        // Bytes allowed per window
        let bytes_per_window = (limit as f64
            * self.window_duration.as_secs_f64()) as u64;

        let mut consumed = self.bytes_in_window.write().await;
        let mut start = self.window_start.write().await;

        // Check if we're in a new window
        if start.elapsed() >= self.window_duration {
            *consumed = 0;
            *start = Instant::now();
        }

        *consumed += bytes as u64;

        // If we've exceeded the budget, sleep until the window ends
        if *consumed >= bytes_per_window {
            let remaining = self.window_duration
                .checked_sub(start.elapsed())
                .unwrap_or(Duration::ZERO);

            if remaining > Duration::ZERO {
                drop(consumed);
                drop(start);
                sleep(remaining).await;
            }
        }
    }

    /// Set new speed limit
    pub async fn set_limit(&self, limit: Option<u64>) {
        *self.limit.write().await = limit;
        *self.bytes_in_window.write().await = 0;
        *self.window_start.write().await = Instant::now();

        tracing::info!(
            "Speed limit set to: {}",
            limit
                .map(|l| format_speed(l as f64))
                .unwrap_or_else(|| "Unlimited".to_string())
        );
    }

    /// Get current limit
    pub async fn get_limit(&self) -> Option<u64> {
        *self.limit.read().await
    }
}

/// Clone the speed limiter
impl Clone for SpeedLimiter {
    fn clone(&self) -> Self {
        Self {
            limit: Arc::clone(&self.limit),
            bytes_in_window: Arc::clone(&self.bytes_in_window),
            window_start: Arc::clone(&self.window_start),
            window_duration: self.window_duration,
        }
    }
}

/// Format speed to human readable string
fn format_speed(bytes_per_sec: f64) -> String {
    const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s", "TB/s"];
    if bytes_per_sec == 0.0 {
        return "0 B/s".to_string();
    }
    let exp = bytes_per_sec.log(1024.0).min(4.0) as usize;
    let value = bytes_per_sec / 1024f64.powi(exp as i32);
    format!("{:.2} {}", value, UNITS[exp])
}
