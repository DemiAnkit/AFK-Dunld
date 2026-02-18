pub mod browser_service;
pub mod clipboard_service;
pub mod config_service;
pub mod file_watcher;
pub mod native_messaging;
pub mod notification_service;
pub mod tray_service;

// Re-export notification types for easier access
#[allow(unused_imports)]
pub use notification_service::{NotificationService, NotificationType};
