// Integration tests for Phase 3 features
// Run with: cargo test --test test_phase3

#[cfg(test)]
mod ftp_tests {
    use std::path::PathBuf;

    // Note: These tests require an FTP server to be running
    // For now, they are marked as ignored and can be run manually
    
    #[tokio::test]
    #[ignore]
    async fn test_ftp_connect() {
        // TODO: Set up test FTP server
        // This would test:
        // 1. Connecting to FTP server
        // 2. Authentication
        // 3. TLS connection if supported
    }

    #[tokio::test]
    #[ignore]
    async fn test_ftp_download_with_resume() {
        // TODO: Test FTP download with resume capability
        // 1. Start a download
        // 2. Stop it midway
        // 3. Resume and verify completion
    }

    #[tokio::test]
    #[ignore]
    async fn test_ftp_list_files() {
        // TODO: Test listing files on FTP server
    }
}

#[cfg(test)]
mod scheduler_tests {
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_schedule_immediate_task() {
        use afk_dunld_lib::core::scheduler::{Scheduler, ScheduledTask};
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a task for immediate execution (1 second ago)
        let task = ScheduledTask {
            id: "test-1".to_string(),
            download_id: "dl-1".to_string(),
            scheduled_time: Utc::now() - Duration::seconds(1),
            repeat_interval: None,
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        let _: Result<(), AppError> = scheduler.start().await;

        // Wait for task to be executed
        let result: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            receiver.recv()
        ).await;
        
        assert!(result.is_ok());
        let executed_task = result.unwrap();
        assert!(executed_task.is_some());
        assert_eq!(executed_task.unwrap().download_id, "dl-1");

        let _: Result<(), AppError> = scheduler.stop().await;
    }

    #[tokio::test]
    async fn test_schedule_future_task() {
        use afk_dunld_lib::core::scheduler::{Scheduler, ScheduledTask};
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a task for 2 seconds in the future
        let task = ScheduledTask {
            id: "test-2".to_string(),
            download_id: "dl-2".to_string(),
            scheduled_time: Utc::now() + Duration::seconds(2),
            repeat_interval: None,
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        let _: Result<(), AppError> = scheduler.start().await;

        // Should not execute immediately
        let result: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            receiver.recv()
        ).await;
        
        assert!(result.is_err()); // Timeout - task hasn't executed yet

        // Wait longer - should execute now
        let result: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            receiver.recv()
        ).await;
        
        assert!(result.is_ok());
        let executed_task = result.unwrap();
        assert!(executed_task.is_some());
        assert_eq!(executed_task.unwrap().download_id, "dl-2");

        let _: Result<(), AppError> = scheduler.stop().await;
    }

    #[tokio::test]
    async fn test_repeating_task() {
        use afk_dunld_lib::core::scheduler::{Scheduler, ScheduledTask, RepeatInterval};
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a repeating task (every 2 seconds)
        let task = ScheduledTask {
            id: "test-3".to_string(),
            download_id: "dl-3".to_string(),
            scheduled_time: Utc::now() - Duration::seconds(1),
            repeat_interval: Some(RepeatInterval::Custom(2)),
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        let _: Result<(), AppError> = scheduler.start().await;

        // Should execute first time
        let result1: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            receiver.recv()
        ).await;
        assert!(result1.is_ok());

        // Should execute second time after interval
        let result2: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            receiver.recv()
        ).await;
        assert!(result2.is_ok());

        let _: Result<(), AppError> = scheduler.stop().await;
    }

    #[tokio::test]
    async fn test_disable_task() {
        use afk_dunld_lib::core::scheduler::{Scheduler, ScheduledTask};
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a disabled task
        let task = ScheduledTask {
            id: "test-4".to_string(),
            download_id: "dl-4".to_string(),
            scheduled_time: Utc::now() - Duration::seconds(1),
            repeat_interval: None,
            enabled: false, // Disabled
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        let _: Result<(), AppError> = scheduler.start().await;

        // Should not execute
        let result: Result<Option<ScheduledTask>, tokio::time::error::Elapsed> = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            receiver.recv()
        ).await;
        
        assert!(result.is_err()); // Timeout - disabled task doesn't execute

        let _: Result<(), AppError> = scheduler.stop().await;
    }
}

#[cfg(test)]
mod torrent_tests {
    use std::path::PathBuf;

    #[tokio::test]
    #[ignore]
    async fn test_parse_magnet_link() {
        use afk_dunld_lib::network::torrent_client::TorrentClient;
        use afk_dunld_lib::utils::error::AppError;
        
        let client = TorrentClient::new(Default::default());
        
        let magnet = "magnet:?xt=urn:btih:1234567890abcdef1234567890abcdef12345678&dn=test";
        let result: Result<String, AppError> = client.add_magnet(magnet).await;
        
        // Should parse successfully (even though download won't work in test)
        assert!(result.is_ok());
        let info_hash = result.unwrap();
        assert_eq!(info_hash, "1234567890abcdef1234567890abcdef12345678");
    }

    #[tokio::test]
    async fn test_invalid_magnet_link() {
        use afk_dunld_lib::network::torrent_client::TorrentClient;
        use afk_dunld_lib::utils::error::AppError;
        
        let client = TorrentClient::new(Default::default());
        
        let invalid_magnet = "not-a-magnet-link";
        let result: Result<String, AppError> = client.add_magnet(invalid_magnet).await;
        
        assert!(result.is_err());
    }
}
