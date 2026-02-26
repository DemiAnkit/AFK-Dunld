// src-tauri/src/network/torrent_helpers.rs
// Helper utilities for torrent management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Torrent priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TorrentPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for TorrentPriority {
    fn default() -> Self {
        TorrentPriority::Normal
    }
}

impl TorrentPriority {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => TorrentPriority::Low,
            1 => TorrentPriority::Normal,
            2 => TorrentPriority::High,
            3 => TorrentPriority::Critical,
            _ => TorrentPriority::Normal,
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

/// Bandwidth limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimit {
    /// Download rate limit in bytes per second (None = unlimited)
    pub download_limit: Option<u64>,
    /// Upload rate limit in bytes per second (None = unlimited)
    pub upload_limit: Option<u64>,
    /// Whether limits are enabled
    pub enabled: bool,
}

impl Default for BandwidthLimit {
    fn default() -> Self {
        Self {
            download_limit: None,
            upload_limit: None,
            enabled: false,
        }
    }
}

impl BandwidthLimit {
    pub fn new(download_limit: Option<u64>, upload_limit: Option<u64>) -> Self {
        Self {
            download_limit,
            upload_limit,
            enabled: download_limit.is_some() || upload_limit.is_some(),
        }
    }

    pub fn unlimited() -> Self {
        Self {
            download_limit: None,
            upload_limit: None,
            enabled: false,
        }
    }

    pub fn set_download_limit(&mut self, limit: Option<u64>) {
        self.download_limit = limit;
        self.enabled = self.download_limit.is_some() || self.upload_limit.is_some();
    }

    pub fn set_upload_limit(&mut self, limit: Option<u64>) {
        self.upload_limit = limit;
        self.enabled = self.download_limit.is_some() || self.upload_limit.is_some();
    }
}

/// Torrent schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentSchedule {
    /// Start time (24-hour format, e.g., "22:00")
    pub start_time: Option<String>,
    /// End time (24-hour format, e.g., "06:00")
    pub end_time: Option<String>,
    /// Days of week (0 = Sunday, 6 = Saturday)
    pub days_of_week: Vec<u8>,
    /// Whether schedule is enabled
    pub enabled: bool,
}

impl Default for TorrentSchedule {
    fn default() -> Self {
        Self {
            start_time: None,
            end_time: None,
            days_of_week: vec![],
            enabled: false,
        }
    }
}

impl TorrentSchedule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_time_range(&mut self, start: String, end: String) {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self.enabled = true;
    }

    pub fn set_days(&mut self, days: Vec<u8>) {
        self.days_of_week = days;
    }

    pub fn is_active_now(&self) -> bool {
        if !self.enabled {
            return true; // No schedule = always active
        }

        use chrono::{Local, Timelike, Datelike};
        let now = Local::now();
        let current_hour = now.hour();
        let current_minute = now.minute();
        let current_day = now.weekday().num_days_from_sunday() as u8;

        // Check day of week
        if !self.days_of_week.is_empty() && !self.days_of_week.contains(&current_day) {
            return false;
        }

        // Check time range
        if let (Some(start), Some(end)) = (&self.start_time, &self.end_time) {
            let current_minutes = current_hour * 60 + current_minute;
            
            if let (Some(start_minutes), Some(end_minutes)) = (
                parse_time(start),
                parse_time(end),
            ) {
                if start_minutes <= end_minutes {
                    // Normal range (e.g., 08:00 to 18:00)
                    return current_minutes >= start_minutes && current_minutes <= end_minutes;
                } else {
                    // Overnight range (e.g., 22:00 to 06:00)
                    return current_minutes >= start_minutes || current_minutes <= end_minutes;
                }
            }
        }

        true // If no valid time range, allow
    }
}

fn parse_time(time_str: &str) -> Option<u32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let hours: u32 = parts[0].parse().ok()?;
    let minutes: u32 = parts[1].parse().ok()?;
    
    if hours >= 24 || minutes >= 60 {
        return None;
    }
    
    Some(hours * 60 + minutes)
}

/// Enhanced torrent metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentMetadata {
    pub info_hash: String,
    pub priority: TorrentPriority,
    pub bandwidth_limit: BandwidthLimit,
    pub schedule: TorrentSchedule,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub added_time: chrono::DateTime<chrono::Utc>,
    pub completed_time: Option<chrono::DateTime<chrono::Utc>>,
    pub save_path: PathBuf,
}

impl TorrentMetadata {
    pub fn new(info_hash: String, save_path: PathBuf) -> Self {
        Self {
            info_hash,
            priority: TorrentPriority::default(),
            bandwidth_limit: BandwidthLimit::default(),
            schedule: TorrentSchedule::default(),
            category: None,
            tags: vec![],
            added_time: chrono::Utc::now(),
            completed_time: None,
            save_path,
        }
    }

    pub fn set_priority(&mut self, priority: TorrentPriority) {
        self.priority = priority;
    }

    pub fn set_bandwidth_limit(&mut self, limit: BandwidthLimit) {
        self.bandwidth_limit = limit;
    }

    pub fn set_schedule(&mut self, schedule: TorrentSchedule) {
        self.schedule = schedule;
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn set_category(&mut self, category: Option<String>) {
        self.category = category;
    }

    pub fn mark_completed(&mut self) {
        if self.completed_time.is_none() {
            self.completed_time = Some(chrono::Utc::now());
        }
    }

    pub fn is_scheduled_active(&self) -> bool {
        self.schedule.is_active_now()
    }
}

/// Torrent filter options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFilter {
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub priority: Option<TorrentPriority>,
    pub state: Option<String>,
}

impl TorrentFilter {
    pub fn new() -> Self {
        Self {
            category: None,
            tags: vec![],
            priority: None,
            state: None,
        }
    }

    pub fn matches(&self, metadata: &TorrentMetadata) -> bool {
        if let Some(ref category) = self.category {
            if metadata.category.as_ref() != Some(category) {
                return false;
            }
        }

        if !self.tags.is_empty() {
            let has_all_tags = self.tags.iter().all(|tag| metadata.tags.contains(tag));
            if !has_all_tags {
                return false;
            }
        }

        if let Some(priority) = self.priority {
            if metadata.priority != priority {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_conversion() {
        assert_eq!(TorrentPriority::from_i32(0), TorrentPriority::Low);
        assert_eq!(TorrentPriority::from_i32(2), TorrentPriority::High);
        assert_eq!(TorrentPriority::High.to_i32(), 2);
    }

    #[test]
    fn test_bandwidth_limit() {
        let mut limit = BandwidthLimit::unlimited();
        assert!(!limit.enabled);

        limit.set_download_limit(Some(1_000_000));
        assert!(limit.enabled);
        assert_eq!(limit.download_limit, Some(1_000_000));
    }

    #[test]
    fn test_schedule_parsing() {
        let time = parse_time("14:30");
        assert_eq!(time, Some(14 * 60 + 30));

        let invalid = parse_time("25:00");
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_metadata_operations() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        metadata.add_tag("important".to_string());
        metadata.add_tag("work".to_string());
        assert_eq!(metadata.tags.len(), 2);

        metadata.remove_tag("work");
        assert_eq!(metadata.tags.len(), 1);
        assert!(metadata.tags.contains(&"important".to_string()));
    }
}
