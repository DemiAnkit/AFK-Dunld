// src-tauri/src/core/speed_limiter.rs

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration, Instant};
use parking_lot::RwLock;

/// Rate limiter using token bucket algorithm
pub struct SpeedLimiter {
    /// Max bytes per second (None = unlimited)
    limit: RwLock<Option<u64>>,
    /// Bytes consumed in current window
    bytes_in_window: RwLock<u64>,
    /// Window start time
    window_start: RwLock<Instant>,
    /// Window duration
    window_duration: Duration,
}

impl SpeedLimiter {
    pub fn new(limit: Option<u64>) -> Self {
        Self {
            limit: RwLock::new(limit),
            bytes_in_window: RwLock::new(0),
            window_start: RwLock::new(Instant::now()),
            window_duration: Duration::from_millis(100), // 100ms windows
        }
    }

    /// Throttle based on how many bytes are being written
    /// This should be called AFTER writing the bytes
    pub async fn throttle(&self, bytes: usize) {
        let limit = *self.limit.read();
        let limit = match limit {
            Some(l) if l > 0 => l,
            _ => return, // No limit
        };

        // Bytes allowed per window
        let bytes_per_window = (limit as f64
            * self.window_duration.as_secs_f64()) as u64;

        let mut consumed = self.bytes_in_window.write();
        let mut start = self.window_start.write();

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
    pub fn set_limit(&self, limit: Option<u64>) {
        *self.limit.write() = limit;
        *self.bytes_in_window.write() = 0;
        *self.window_start.write() = Instant::now();

        tracing::info!(
            "Speed limit set to: {}",
            limit
                .map(|l| crate::utils::format_utils::format_speed(l as f64))
                .unwrap_or_else(|| "Unlimited".to_string())
        );
    }

    /// Get current limit
    pub fn get_limit(&self) -> Option<u64> {
        *self.limit.read()
    }
}