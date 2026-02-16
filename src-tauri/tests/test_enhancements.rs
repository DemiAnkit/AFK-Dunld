// Integration tests for Option 2 enhancements
// Run with: cargo test --test test_enhancements

#[cfg(test)]
mod sftp_tests {
    use afk_dunld_lib::network::sftp_client::SftpClient;
    use std::path::PathBuf;

    #[test]
    fn test_parse_sftp_url() {
        let (client, path) = SftpClient::from_url(
            "sftp://user@example.com:2222/path/to/file.zip",
            Some("password".to_string()),
            None
        ).unwrap();
        
        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 2222);
        assert_eq!(client.username, "user");
        assert_eq!(path, "/path/to/file.zip");
    }

    #[test]
    fn test_parse_sftp_url_with_password_in_url() {
        let (client, _) = SftpClient::from_url(
            "sftp://user:urlpass@example.com/file.zip",
            Some("otherpass".to_string()),
            None
        ).unwrap();
        
        // URL password should take precedence
        assert_eq!(client.password, Some("urlpass".to_string()));
    }

    #[test]
    fn test_parse_sftp_default_port() {
        let (client, _) = SftpClient::from_url(
            "sftp://example.com/file.zip",
            None,
            None
        ).unwrap();
        
        assert_eq!(client.port, 22);
    }

    #[test]
    fn test_invalid_scheme() {
        let result = SftpClient::from_url(
            "http://example.com/file.zip",
            None,
            None
        );
        
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod category_tests {
    use afk_dunld_lib::core::category::Category;

    #[test]
    fn test_detect_from_extension_videos() {
        assert_eq!(Category::detect_from_extension("mp4"), "videos");
        assert_eq!(Category::detect_from_extension("MP4"), "videos");
        assert_eq!(Category::detect_from_extension("avi"), "videos");
        assert_eq!(Category::detect_from_extension("mkv"), "videos");
    }

    #[test]
    fn test_detect_from_extension_documents() {
        assert_eq!(Category::detect_from_extension("pdf"), "documents");
        assert_eq!(Category::detect_from_extension("doc"), "documents");
        assert_eq!(Category::detect_from_extension("docx"), "documents");
        assert_eq!(Category::detect_from_extension("txt"), "documents");
    }

    #[test]
    fn test_detect_from_extension_music() {
        assert_eq!(Category::detect_from_extension("mp3"), "music");
        assert_eq!(Category::detect_from_extension("wav"), "music");
        assert_eq!(Category::detect_from_extension("flac"), "music");
    }

    #[test]
    fn test_detect_from_extension_images() {
        assert_eq!(Category::detect_from_extension("jpg"), "images");
        assert_eq!(Category::detect_from_extension("png"), "images");
        assert_eq!(Category::detect_from_extension("gif"), "images");
    }

    #[test]
    fn test_detect_from_extension_software() {
        assert_eq!(Category::detect_from_extension("exe"), "software");
        assert_eq!(Category::detect_from_extension("dmg"), "software");
        assert_eq!(Category::detect_from_extension("deb"), "software");
    }

    #[test]
    fn test_detect_from_extension_compressed() {
        assert_eq!(Category::detect_from_extension("zip"), "compressed");
        assert_eq!(Category::detect_from_extension("rar"), "compressed");
        assert_eq!(Category::detect_from_extension("7z"), "compressed");
    }

    #[test]
    fn test_detect_from_extension_unknown() {
        assert_eq!(Category::detect_from_extension("xyz"), "default");
        assert_eq!(Category::detect_from_extension(""), "default");
    }

    #[test]
    fn test_detect_from_mime_video() {
        assert_eq!(Category::detect_from_mime("video/mp4"), "videos");
        assert_eq!(Category::detect_from_mime("video/x-matroska"), "videos");
    }

    #[test]
    fn test_detect_from_mime_audio() {
        assert_eq!(Category::detect_from_mime("audio/mpeg"), "music");
        assert_eq!(Category::detect_from_mime("audio/wav"), "music");
    }

    #[test]
    fn test_detect_from_mime_image() {
        assert_eq!(Category::detect_from_mime("image/png"), "images");
        assert_eq!(Category::detect_from_mime("image/jpeg"), "images");
    }

    #[test]
    fn test_detect_from_mime_document() {
        assert_eq!(Category::detect_from_mime("application/pdf"), "documents");
        assert_eq!(Category::detect_from_mime("application/msword"), "documents");
    }

    #[test]
    fn test_detect_from_mime_compressed() {
        assert_eq!(Category::detect_from_mime("application/zip"), "compressed");
        assert_eq!(Category::detect_from_mime("application/x-compressed"), "compressed");
    }

    #[test]
    fn test_category_creation() {
        let category = Category::new(
            "Test Category".to_string(),
            Some("#FF0000".to_string()),
            Some("star".to_string()),
            Some(PathBuf::from("/test/path"))
        );

        assert_eq!(category.name, "Test Category");
        assert_eq!(category.color, Some("#FF0000".to_string()));
        assert_eq!(category.icon, Some("star".to_string()));
        assert_eq!(category.save_path, Some(PathBuf::from("/test/path")));
        assert!(category.created_at > 0);
        assert!(category.updated_at > 0);
    }

    #[test]
    fn test_default_category() {
        let category = Category::default();
        
        assert_eq!(category.id, "default");
        assert_eq!(category.name, "Default");
        assert_eq!(category.color, Some("#6B7280".to_string()));
        assert_eq!(category.icon, Some("folder".to_string()));
    }
}

#[cfg(test)]
mod ftp_directory_tests {
    use afk_dunld_lib::network::ftp_client::FtpClient;

    #[test]
    fn test_ftp_client_creation() {
        let client = FtpClient::new(
            "example.com".to_string(),
            21,
            Some("user".to_string()),
            Some("pass".to_string()),
            false
        );

        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 21);
        assert_eq!(client.username, Some("user".to_string()));
        assert!(!client.use_tls);
    }

    #[test]
    fn test_parse_ftp_url_with_credentials() {
        let (client, path) = FtpClient::from_url(
            "ftp://user:pass@example.com:2121/path/to/file.zip"
        ).unwrap();
        
        assert_eq!(client.host, "example.com");
        assert_eq!(client.port, 2121);
        assert_eq!(client.username, Some("user".to_string()));
        assert_eq!(client.password, Some("pass".to_string()));
        assert_eq!(path, "/path/to/file.zip");
    }

    #[test]
    fn test_parse_ftps_url() {
        let (client, _) = FtpClient::from_url("ftps://example.com/file.zip").unwrap();
        
        assert!(client.use_tls);
        assert_eq!(client.port, 21);
    }
}

#[cfg(test)]
mod torrent_integration_tests {
    use afk_dunld_lib::network::torrent_client_librqbit::TorrentConfig;
    use std::path::PathBuf;

    #[test]
    fn test_torrent_config_default() {
        let config = TorrentConfig::default();
        
        assert_eq!(config.download_dir, PathBuf::from("downloads"));
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.seed_ratio, 2.0);
        assert!(config.dht_enabled);
        assert!(config.pex_enabled);
    }

    #[test]
    fn test_torrent_config_custom() {
        let config = TorrentConfig {
            download_dir: PathBuf::from("/custom/path"),
            max_connections: 100,
            max_upload_rate: Some(1024 * 100), // 100 KB/s
            max_download_rate: Some(1024 * 500), // 500 KB/s
            seed_ratio: 1.5,
            dht_enabled: false,
            pex_enabled: false,
        };

        assert_eq!(config.download_dir, PathBuf::from("/custom/path"));
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.max_upload_rate, Some(102400));
        assert_eq!(config.seed_ratio, 1.5);
        assert!(!config.dht_enabled);
    }
}

#[cfg(test)]
mod scheduler_integration_tests {
    use afk_dunld_lib::core::scheduler::{Scheduler, ScheduledTask, RepeatInterval};
    use chrono::{Duration, Utc};
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_schedule_and_retrieve() {
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, _receiver) = Scheduler::new();
        
        let task = ScheduledTask {
            id: "test-1".to_string(),
            download_id: "dl-1".to_string(),
            scheduled_time: Utc::now() + Duration::hours(1),
            repeat_interval: None,
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task.clone()).await;
        
        let retrieved = scheduler.get_task("test-1").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().download_id, "dl-1");
    }

    #[tokio::test]
    async fn test_list_all_tasks() {
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, _receiver) = Scheduler::new();
        
        for i in 0..3 {
            let task = ScheduledTask {
                id: format!("test-{}", i),
                download_id: format!("dl-{}", i),
                scheduled_time: Utc::now() + Duration::hours(i),
                repeat_interval: None,
                enabled: true,
            };
            let _: Result<(), AppError> = scheduler.add_task(task).await;
        }
        
        let all_tasks = scheduler.get_all_tasks().await;
        assert_eq!(all_tasks.len(), 3);
    }

    #[tokio::test]
    async fn test_remove_task() {
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, _receiver) = Scheduler::new();
        
        let task = ScheduledTask {
            id: "test-remove".to_string(),
            download_id: "dl-remove".to_string(),
            scheduled_time: Utc::now() + Duration::hours(1),
            repeat_interval: None,
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        
        assert!(scheduler.get_task("test-remove").await.is_some());
        
        let _: Result<(), AppError> = scheduler.remove_task("test-remove").await;
        
        assert!(scheduler.get_task("test-remove").await.is_none());
    }

    #[tokio::test]
    async fn test_update_task() {
        use afk_dunld_lib::utils::error::AppError;
        
        let (scheduler, _receiver) = Scheduler::new();
        
        let task = ScheduledTask {
            id: "test-update".to_string(),
            download_id: "dl-update".to_string(),
            scheduled_time: Utc::now() + Duration::hours(1),
            repeat_interval: None,
            enabled: true,
        };

        let _: Result<(), AppError> = scheduler.add_task(task).await;
        
        let mut updated_task = scheduler.get_task("test-update").await.unwrap();
        updated_task.enabled = false;
        updated_task.repeat_interval = Some(RepeatInterval::Daily);
        
        let _: Result<(), AppError> = scheduler.update_task(updated_task).await;
        
        let retrieved = scheduler.get_task("test-update").await.unwrap();
        assert!(!retrieved.enabled);
        assert!(matches!(retrieved.repeat_interval, Some(RepeatInterval::Daily)));
    }
}

#[cfg(test)]
mod end_to_end_tests {
    #[tokio::test]
    #[ignore] // Requires actual network resources
    async fn test_full_download_workflow() {
        // This would test a complete download workflow:
        // 1. Create category
        // 2. Add download
        // 3. Auto-categorize
        // 4. Schedule download
        // 5. Monitor progress
        // 6. Verify completion
    }
}
