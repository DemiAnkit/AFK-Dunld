// src-tauri/src/core/speed_tracker.rs

use std::collections::VecDeque;
use std::time::Instant;
use crate::utils::constants::SPEED_WINDOW_SECONDS;

/// Tracks download speed using a sliding window
#[derive(Debug)]
pub struct SpeedTracker {
    /// (timestamp, bytes_downloaded_at_that_point)
    samples: VecDeque<(Instant, u64)>,
    /// Window duration for averaging
    window: f64,
    /// Total bytes downloaded
    total_bytes: u64,
    /// Cached speed value
    cached_speed: f64,
    /// Last calculation time
    last_calc: Instant,
}

impl SpeedTracker {
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(100),
            window: SPEED_WINDOW_SECONDS,
            total_bytes: 0,
            cached_speed: 0.0,
            last_calc: Instant::now(),
        }
    }

    pub fn with_window(window_seconds: f64) -> Self {
        Self {
            window: window_seconds,
            ..Self::new()
        }
    }

    /// Record bytes downloaded
    pub fn add_bytes(&mut self, bytes: u64) {
        self.total_bytes += bytes;
        let now = Instant::now();
        self.samples.push_back((now, self.total_bytes));
        self.cleanup(now);
    }

    /// Get current speed in bytes/sec (sliding window average)
    pub fn speed(&mut self) -> f64 {
        let now = Instant::now();

        // Recalculate at most every 100ms
        if now.duration_since(self.last_calc).as_millis() < 100 {
            return self.cached_speed;
        }

        self.cleanup(now);
        self.last_calc = now;

        if self.samples.len() < 2 {
            self.cached_speed = 0.0;
            return 0.0;
        }

        let oldest = &self.samples[0];
        let newest = self.samples.back().unwrap();
        let elapsed = newest.0.duration_since(oldest.0).as_secs_f64();

        if elapsed < 0.001 {
            self.cached_speed = 0.0;
            return 0.0;
        }

        let bytes_diff = newest.1 - oldest.1;
        self.cached_speed = bytes_diff as f64 / elapsed;
        self.cached_speed
    }

    /// Calculate ETA in seconds
    pub fn eta(&mut self, remaining_bytes: u64) -> Option<u64> {
        let speed = self.speed();
        if speed <= 0.0 {
            return None;
        }
        Some((remaining_bytes as f64 / speed) as u64)
    }

    /// Get total bytes downloaded
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// Reset the tracker
    pub fn reset(&mut self) {
        self.samples.clear();
        self.total_bytes = 0;
        self.cached_speed = 0.0;
    }

    /// Remove samples outside the window
    fn cleanup(&mut self, now: Instant) {
        let cutoff = now - std::time::Duration::from_secs_f64(self.window);
        while let Some(front) = self.samples.front() {
            if front.0 < cutoff {
                self.samples.pop_front();
            } else {
                break;
            }
        }
    }
}

/// Aggregate speed tracker for multiple concurrent downloads
pub struct GlobalSpeedTracker {
    speeds: parking_lot::RwLock<std::collections::HashMap<uuid::Uuid, f64>>,
}

impl GlobalSpeedTracker {
    pub fn new() -> Self {
        Self {
            speeds: parking_lot::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn update(&self, id: uuid::Uuid, speed: f64) {
        self.speeds.write().insert(id, speed);
    }

    pub fn remove(&self, id: &uuid::Uuid) {
        self.speeds.write().remove(id);
    }

    pub fn total_speed(&self) -> f64 {
        self.speeds.read().values().sum()
    }

    pub fn active_count(&self) -> usize {
        self.speeds.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_speed_tracker() {
        let mut tracker = SpeedTracker::with_window(5.0);

        // Simulate downloading 1MB/s
        for _ in 0..10 {
            tracker.add_bytes(100_000);
            thread::sleep(Duration::from_millis(100));
        }

        let speed = tracker.speed();
        // Should be roughly 1MB/s (with some tolerance)
        assert!(speed > 500_000.0, "Speed too low: {}", speed);
        assert!(speed < 2_000_000.0, "Speed too high: {}", speed);
    }
}