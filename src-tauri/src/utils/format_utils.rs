// src-tauri/src/utils/format_utils.rs

/// Format bytes into human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let exp = (bytes as f64).log(1024.0).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    let value = bytes as f64 / 1024f64.powi(exp as i32);
    format!("{:.2} {}", value, UNITS[exp])
}

/// Format speed in bytes/sec
pub fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec <= 0.0 {
        return "0 B/s".to_string();
    }
    format!("{}/s", format_bytes(bytes_per_sec as u64))
}

/// Format duration in seconds to human-readable
pub fn format_eta(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    if seconds < 3600 {
        let m = seconds / 60;
        let s = seconds % 60;
        return format!("{}m {}s", m, s);
    }
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    format!("{}h {}m", h, m)
}