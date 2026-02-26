// src-tauri/tests/torrent_integration_test.rs
// Integration tests for torrent functionality

#[cfg(test)]
mod torrent_tests {
    use afk_dunld_lib::network::bencode_parser::{TorrentFile, MagnetLink};
    use afk_dunld_lib::network::torrent_helpers::{
        TorrentPriority, BandwidthLimit, TorrentSchedule, TorrentMetadata,
    };
    use std::path::PathBuf;

    #[test]
    fn test_magnet_link_parsing() {
        let magnet = "magnet:?xt=urn:btih:1234567890abcdef1234567890abcdef12345678&dn=Test%20File&tr=http://tracker.example.com:8080/announce";
        
        let result = MagnetLink::parse(magnet);
        assert!(result.is_ok(), "Should parse valid magnet link");
        
        let parsed = result.unwrap();
        assert_eq!(parsed.info_hash, "1234567890abcdef1234567890abcdef12345678");
        assert_eq!(parsed.display_name, Some("Test File".to_string()));
        assert_eq!(parsed.trackers.len(), 1);
        assert_eq!(parsed.trackers[0], "http://tracker.example.com:8080/announce");
    }

    #[test]
    fn test_magnet_link_parsing_minimal() {
        let magnet = "magnet:?xt=urn:btih:abcdef1234567890abcdef1234567890abcdef12";
        
        let result = MagnetLink::parse(magnet);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.info_hash, "abcdef1234567890abcdef1234567890abcdef12");
        assert_eq!(parsed.display_name, None);
        assert_eq!(parsed.trackers.len(), 0);
    }

    #[test]
    fn test_magnet_link_parsing_multiple_trackers() {
        let magnet = "magnet:?xt=urn:btih:1234567890abcdef1234567890abcdef12345678&tr=http://tracker1.com&tr=http://tracker2.com&tr=udp://tracker3.com:6969";
        
        let result = MagnetLink::parse(magnet);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.trackers.len(), 3);
    }

    #[test]
    fn test_magnet_link_parsing_invalid() {
        let invalid_magnets = vec![
            "not a magnet link",
            "http://example.com",
            "magnet:?dn=NoInfoHash",
            "",
        ];

        for magnet in invalid_magnets {
            let result = MagnetLink::parse(magnet);
            assert!(result.is_err(), "Should reject invalid magnet: {}", magnet);
        }
    }

    #[test]
    fn test_priority_conversion() {
        assert_eq!(TorrentPriority::from_i32(0), TorrentPriority::Low);
        assert_eq!(TorrentPriority::from_i32(1), TorrentPriority::Normal);
        assert_eq!(TorrentPriority::from_i32(2), TorrentPriority::High);
        assert_eq!(TorrentPriority::from_i32(3), TorrentPriority::Critical);
        assert_eq!(TorrentPriority::from_i32(999), TorrentPriority::Normal); // Default

        assert_eq!(TorrentPriority::Low.to_i32(), 0);
        assert_eq!(TorrentPriority::Normal.to_i32(), 1);
        assert_eq!(TorrentPriority::High.to_i32(), 2);
        assert_eq!(TorrentPriority::Critical.to_i32(), 3);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(TorrentPriority::Low < TorrentPriority::Normal);
        assert!(TorrentPriority::Normal < TorrentPriority::High);
        assert!(TorrentPriority::High < TorrentPriority::Critical);
    }

    #[test]
    fn test_bandwidth_limit_creation() {
        let unlimited = BandwidthLimit::unlimited();
        assert!(!unlimited.enabled);
        assert_eq!(unlimited.download_limit, None);
        assert_eq!(unlimited.upload_limit, None);

        let limited = BandwidthLimit::new(Some(1_000_000), Some(500_000));
        assert!(limited.enabled);
        assert_eq!(limited.download_limit, Some(1_000_000));
        assert_eq!(limited.upload_limit, Some(500_000));
    }

    #[test]
    fn test_bandwidth_limit_modification() {
        let mut limit = BandwidthLimit::unlimited();
        assert!(!limit.enabled);

        limit.set_download_limit(Some(2_000_000));
        assert!(limit.enabled);
        assert_eq!(limit.download_limit, Some(2_000_000));

        limit.set_upload_limit(Some(1_000_000));
        assert_eq!(limit.upload_limit, Some(1_000_000));

        limit.set_download_limit(None);
        limit.set_upload_limit(None);
        assert!(!limit.enabled);
    }

    #[test]
    fn test_schedule_creation() {
        let mut schedule = TorrentSchedule::default();
        assert!(!schedule.enabled);
        assert!(schedule.days_of_week.is_empty());

        schedule.set_time_range("22:00".to_string(), "06:00".to_string());
        assert!(schedule.enabled);
        assert_eq!(schedule.start_time, Some("22:00".to_string()));
        assert_eq!(schedule.end_time, Some("06:00".to_string()));

        schedule.set_days(vec![0, 6]); // Sunday and Saturday
        assert_eq!(schedule.days_of_week, vec![0, 6]);
    }

    #[test]
    fn test_schedule_always_active_when_disabled() {
        let schedule = TorrentSchedule::default();
        assert!(schedule.is_active_now()); // Disabled schedule = always active
    }

    #[test]
    fn test_torrent_metadata_creation() {
        let metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        assert_eq!(metadata.info_hash, "test_hash");
        assert_eq!(metadata.priority, TorrentPriority::Normal);
        assert!(!metadata.bandwidth_limit.enabled);
        assert!(!metadata.schedule.enabled);
        assert_eq!(metadata.category, None);
        assert!(metadata.tags.is_empty());
        assert_eq!(metadata.completed_time, None);
    }

    #[test]
    fn test_torrent_metadata_tag_management() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        metadata.add_tag("important".to_string());
        assert_eq!(metadata.tags.len(), 1);
        assert!(metadata.tags.contains(&"important".to_string()));

        metadata.add_tag("work".to_string());
        assert_eq!(metadata.tags.len(), 2);

        // Adding duplicate should not increase count
        metadata.add_tag("important".to_string());
        assert_eq!(metadata.tags.len(), 2);

        metadata.remove_tag("work");
        assert_eq!(metadata.tags.len(), 1);
        assert!(!metadata.tags.contains(&"work".to_string()));
    }

    #[test]
    fn test_torrent_metadata_priority() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        assert_eq!(metadata.priority, TorrentPriority::Normal);

        metadata.set_priority(TorrentPriority::High);
        assert_eq!(metadata.priority, TorrentPriority::High);

        metadata.set_priority(TorrentPriority::Critical);
        assert_eq!(metadata.priority, TorrentPriority::Critical);
    }

    #[test]
    fn test_torrent_metadata_category() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        assert_eq!(metadata.category, None);

        metadata.set_category(Some("movies".to_string()));
        assert_eq!(metadata.category, Some("movies".to_string()));

        metadata.set_category(None);
        assert_eq!(metadata.category, None);
    }

    #[test]
    fn test_torrent_metadata_bandwidth_limit() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        let limit = BandwidthLimit::new(Some(1_000_000), Some(500_000));
        metadata.set_bandwidth_limit(limit.clone());

        assert_eq!(metadata.bandwidth_limit.download_limit, Some(1_000_000));
        assert_eq!(metadata.bandwidth_limit.upload_limit, Some(500_000));
        assert!(metadata.bandwidth_limit.enabled);
    }

    #[test]
    fn test_torrent_metadata_schedule() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        let mut schedule = TorrentSchedule::default();
        schedule.set_time_range("22:00".to_string(), "06:00".to_string());
        schedule.set_days(vec![1, 2, 3, 4, 5]); // Weekdays
        
        metadata.set_schedule(schedule);

        assert!(metadata.schedule.enabled);
        assert_eq!(metadata.schedule.start_time, Some("22:00".to_string()));
        assert_eq!(metadata.schedule.end_time, Some("06:00".to_string()));
        assert_eq!(metadata.schedule.days_of_week.len(), 5);
    }

    #[test]
    fn test_torrent_metadata_completion() {
        let mut metadata = TorrentMetadata::new(
            "test_hash".to_string(),
            PathBuf::from("/downloads"),
        );

        assert_eq!(metadata.completed_time, None);

        metadata.mark_completed();
        assert!(metadata.completed_time.is_some());

        // Calling again should not change the time
        let first_completion = metadata.completed_time;
        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.mark_completed();
        assert_eq!(metadata.completed_time, first_completion);
    }

    // Database integration tests
    #[tokio::test]
    async fn test_torrent_database_roundtrip() {
        use afk_dunld_lib::database::db::Database;
        use afk_dunld_lib::database::torrent_queries::{save_torrent, load_torrent};
        use afk_dunld_lib::network::torrent_client_librqbit::{TorrentInfo, TorrentStats, TorrentFile};
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let db = Database::new(&temp_dir.path().to_path_buf()).await.unwrap();
        db.run_migrations().await.unwrap();

        let info = TorrentInfo {
            info_hash: "test_hash_123".to_string(),
            name: "Test Torrent".to_string(),
            total_size: 1024 * 1024 * 100, // 100 MB
            piece_length: 256 * 1024,
            num_pieces: 400,
            files: vec![
                TorrentFile {
                    path: PathBuf::from("file1.txt"),
                    size: 1024 * 1024 * 50,
                },
                TorrentFile {
                    path: PathBuf::from("file2.txt"),
                    size: 1024 * 1024 * 50,
                },
            ],
        };

        let stats = TorrentStats {
            downloaded: 1024 * 1024 * 25,
            uploaded: 1024 * 1024 * 10,
            download_rate: 1024 * 100,
            upload_rate: 1024 * 50,
            peers: 15,
            seeders: 5,
            progress: 0.25,
            eta: Some(300),
        };

        let mut metadata = TorrentMetadata::new(
            "test_hash_123".to_string(),
            PathBuf::from("/downloads/torrents"),
        );
        metadata.add_tag("test".to_string());
        metadata.add_tag("integration".to_string());
        metadata.set_category(Some("tests".to_string()));
        metadata.set_priority(TorrentPriority::High);

        // Save to database
        let result = save_torrent(db.pool(), &info, &stats, &metadata).await;
        assert!(result.is_ok(), "Failed to save torrent: {:?}", result.err());

        // Load from database
        let loaded = load_torrent(db.pool(), "test_hash_123").await.unwrap();
        assert!(loaded.is_some(), "Torrent should be found");

        let (torrent_row, files, tags, bandwidth, schedule) = loaded.unwrap();
        assert_eq!(torrent_row.info_hash, "test_hash_123");
        assert_eq!(torrent_row.name, "Test Torrent");
        assert_eq!(torrent_row.total_size, 1024 * 1024 * 100);
        assert_eq!(torrent_row.priority, 2); // High priority
        assert_eq!(torrent_row.category, Some("tests".to_string()));
        assert_eq!(files.len(), 2);
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"test".to_string()));
        assert!(tags.contains(&"integration".to_string()));
    }

    #[tokio::test]
    async fn test_torrent_database_update() {
        use afk_dunld_lib::database::db::Database;
        use afk_dunld_lib::database::torrent_queries::{save_torrent, update_torrent_stats, load_torrent};
        use afk_dunld_lib::network::torrent_client_librqbit::{TorrentInfo, TorrentStats, TorrentFile};
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let db = Database::new(&temp_dir.path().to_path_buf()).await.unwrap();
        db.run_migrations().await.unwrap();

        let info = TorrentInfo {
            info_hash: "update_test".to_string(),
            name: "Update Test".to_string(),
            total_size: 1000,
            piece_length: 100,
            num_pieces: 10,
            files: vec![],
        };

        let initial_stats = TorrentStats {
            downloaded: 100,
            uploaded: 50,
            download_rate: 10,
            upload_rate: 5,
            peers: 2,
            seeders: 1,
            progress: 0.1,
            eta: Some(900),
        };

        let metadata = TorrentMetadata::new(
            "update_test".to_string(),
            PathBuf::from("/downloads"),
        );

        // Initial save
        save_torrent(db.pool(), &info, &initial_stats, &metadata).await.unwrap();

        // Update stats
        let updated_stats = TorrentStats {
            downloaded: 500,
            uploaded: 250,
            download_rate: 50,
            upload_rate: 25,
            peers: 10,
            seeders: 5,
            progress: 0.5,
            eta: Some(500),
        };

        update_torrent_stats(db.pool(), "update_test", &updated_stats).await.unwrap();

        // Verify update
        let loaded = load_torrent(db.pool(), "update_test").await.unwrap().unwrap();
        let (torrent_row, _, _, _, _) = loaded;
        
        assert_eq!(torrent_row.downloaded_size, 500);
        assert_eq!(torrent_row.uploaded_size, 250);
        assert_eq!(torrent_row.peers, 10);
        assert_eq!(torrent_row.seeders, 5);
        assert!((torrent_row.progress - 0.5).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_torrent_database_delete() {
        use afk_dunld_lib::database::db::Database;
        use afk_dunld_lib::database::torrent_queries::{save_torrent, delete_torrent, load_torrent};
        use afk_dunld_lib::network::torrent_client_librqbit::{TorrentInfo, TorrentStats};
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let db = Database::new(&temp_dir.path().to_path_buf()).await.unwrap();
        db.run_migrations().await.unwrap();

        let info = TorrentInfo {
            info_hash: "delete_test".to_string(),
            name: "Delete Test".to_string(),
            total_size: 1000,
            piece_length: 100,
            num_pieces: 10,
            files: vec![],
        };

        let stats = TorrentStats {
            downloaded: 0,
            uploaded: 0,
            download_rate: 0,
            upload_rate: 0,
            peers: 0,
            seeders: 0,
            progress: 0.0,
            eta: None,
        };

        let metadata = TorrentMetadata::new(
            "delete_test".to_string(),
            PathBuf::from("/downloads"),
        );

        // Save
        save_torrent(db.pool(), &info, &stats, &metadata).await.unwrap();

        // Verify exists
        let loaded = load_torrent(db.pool(), "delete_test").await.unwrap();
        assert!(loaded.is_some());

        // Delete
        delete_torrent(db.pool(), "delete_test").await.unwrap();

        // Verify deleted
        let loaded_after = load_torrent(db.pool(), "delete_test").await.unwrap();
        assert!(loaded_after.is_none());
    }
}
