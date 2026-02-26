// src-tauri/src/database/torrent_queries.rs
// Database queries for torrent persistence

use sqlx::{SqlitePool, Row};
use crate::database::models::{TorrentRow, TorrentFileRow, TorrentBandwidthRow, TorrentScheduleRow};
use crate::utils::error::AppError;
use crate::network::torrent_client_librqbit::{TorrentInfo, TorrentStats, TorrentFile};
use crate::network::torrent_helpers::{TorrentMetadata, BandwidthLimit, TorrentSchedule};
use crate::network::torrent_advanced::{WebSeed, WebSeedType, EncryptionConfig, EncryptionMode};

/// Save or update torrent metadata in database
pub async fn save_torrent(
    pool: &SqlitePool,
    info: &TorrentInfo,
    stats: &TorrentStats,
    metadata: &TorrentMetadata,
) -> Result<(), AppError> {
    let state = "Downloading"; // Convert TorrentState to string

    sqlx::query(
        r#"
        INSERT INTO torrents (
            info_hash, name, total_size, piece_length, num_pieces, save_path,
            priority, category, added_time, completed_time, state,
            downloaded_size, uploaded_size, download_rate, upload_rate,
            peers, seeders, progress, eta
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(info_hash) DO UPDATE SET
            name = excluded.name,
            total_size = excluded.total_size,
            priority = excluded.priority,
            category = excluded.category,
            completed_time = excluded.completed_time,
            state = excluded.state,
            downloaded_size = excluded.downloaded_size,
            uploaded_size = excluded.uploaded_size,
            download_rate = excluded.download_rate,
            upload_rate = excluded.upload_rate,
            peers = excluded.peers,
            seeders = excluded.seeders,
            progress = excluded.progress,
            eta = excluded.eta
        "#,
    )
    .bind(&info.info_hash)
    .bind(&info.name)
    .bind(info.total_size as i64)
    .bind(info.piece_length as i64)
    .bind(info.num_pieces as i64)
    .bind(metadata.save_path.to_string_lossy().to_string())
    .bind(metadata.priority.to_i32())
    .bind(&metadata.category)
    .bind(metadata.added_time.to_rfc3339())
    .bind(metadata.completed_time.map(|t| t.to_rfc3339()))
    .bind(state)
    .bind(stats.downloaded as i64)
    .bind(stats.uploaded as i64)
    .bind(stats.download_rate as i64)
    .bind(stats.upload_rate as i64)
    .bind(stats.peers as i32)
    .bind(stats.seeders as i32)
    .bind(stats.progress)
    .bind(stats.eta.map(|e| e as i64))
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to save torrent: {}", e)))?;

    // Save files
    save_torrent_files(pool, &info.info_hash, &info.files).await?;

    // Save tags
    save_torrent_tags(pool, &info.info_hash, &metadata.tags).await?;

    // Save bandwidth limits
    save_bandwidth_limit(pool, &info.info_hash, &metadata.bandwidth_limit).await?;

    // Save schedule
    save_schedule(pool, &info.info_hash, &metadata.schedule).await?;

    Ok(())
}

/// Save web seeds for a torrent
pub async fn save_web_seeds(
    pool: &SqlitePool,
    info_hash: &str,
    web_seeds: &[WebSeed],
) -> Result<(), AppError> {
    // Delete existing web seeds
    sqlx::query("DELETE FROM torrent_web_seeds WHERE info_hash = ?")
        .bind(info_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete web seeds: {}", e)))?;

    // Insert new web seeds
    for seed in web_seeds {
        let seed_type_str = match seed.seed_type {
            WebSeedType::GetRight => "GetRight",
            WebSeedType::WebSeed => "WebSeed",
        };

        sqlx::query(
            "INSERT INTO torrent_web_seeds (info_hash, url, seed_type) VALUES (?, ?, ?)"
        )
        .bind(info_hash)
        .bind(&seed.url)
        .bind(seed_type_str)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save web seed: {}", e)))?;
    }

    Ok(())
}

/// Load web seeds for a torrent
pub async fn load_web_seeds(
    pool: &SqlitePool,
    info_hash: &str,
) -> Result<Vec<WebSeed>, AppError> {
    let rows = sqlx::query("SELECT url, seed_type FROM torrent_web_seeds WHERE info_hash = ?")
        .bind(info_hash)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to load web seeds: {}", e)))?;

    let mut web_seeds = Vec::new();
    for row in rows {
        let url: String = row.get("url");
        let seed_type_str: String = row.get("seed_type");
        let seed_type = match seed_type_str.as_str() {
            "GetRight" => WebSeedType::GetRight,
            _ => WebSeedType::WebSeed,
        };

        web_seeds.push(WebSeed { url, seed_type });
    }

    Ok(web_seeds)
}

/// Save encryption config for a torrent
pub async fn save_encryption_config(
    pool: &SqlitePool,
    info_hash: &str,
    encryption: &EncryptionConfig,
) -> Result<(), AppError> {
    let mode_str = match encryption.mode {
        EncryptionMode::Disabled => "Disabled",
        EncryptionMode::Enabled => "Enabled",
        EncryptionMode::Required => "Required",
    };

    sqlx::query(
        r#"
        INSERT INTO torrent_encryption (info_hash, enabled, mode, prefer_encrypted)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(info_hash) DO UPDATE SET
            enabled = excluded.enabled,
            mode = excluded.mode,
            prefer_encrypted = excluded.prefer_encrypted
        "#,
    )
    .bind(info_hash)
    .bind(encryption.enabled)
    .bind(mode_str)
    .bind(encryption.prefer_encrypted)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to save encryption config: {}", e)))?;

    Ok(())
}

/// Load encryption config for a torrent
pub async fn load_encryption_config(
    pool: &SqlitePool,
    info_hash: &str,
) -> Result<EncryptionConfig, AppError> {
    let row = sqlx::query("SELECT enabled, mode, prefer_encrypted FROM torrent_encryption WHERE info_hash = ?")
        .bind(info_hash)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to load encryption config: {}", e)))?;

    if let Some(row) = row {
        let enabled: bool = row.get("enabled");
        let mode_str: String = row.get("mode");
        let prefer_encrypted: bool = row.get("prefer_encrypted");

        let mode = match mode_str.as_str() {
            "Disabled" => EncryptionMode::Disabled,
            "Required" => EncryptionMode::Required,
            _ => EncryptionMode::Enabled,
        };

        Ok(EncryptionConfig {
            enabled,
            mode,
            prefer_encrypted,
        })
    } else {
        Ok(EncryptionConfig::default())
    }
}

/// Save blocked IPs for a torrent
pub async fn save_blocked_ips(
    pool: &SqlitePool,
    info_hash: &str,
    ips: &[String],
) -> Result<(), AppError> {
    // Delete existing blocked IPs
    sqlx::query("DELETE FROM torrent_ip_filter WHERE info_hash = ?")
        .bind(info_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete blocked IPs: {}", e)))?;

    // Insert new blocked IPs
    for ip in ips {
        sqlx::query("INSERT INTO torrent_ip_filter (info_hash, ip) VALUES (?, ?)")
            .bind(info_hash)
            .bind(ip)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save blocked IP: {}", e)))?;
    }

    Ok(())
}

/// Load blocked IPs for a torrent
pub async fn load_blocked_ips(
    pool: &SqlitePool,
    info_hash: &str,
) -> Result<Vec<String>, AppError> {
    let rows = sqlx::query("SELECT ip FROM torrent_ip_filter WHERE info_hash = ?")
        .bind(info_hash)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to load blocked IPs: {}", e)))?;

    Ok(rows.iter().map(|row| row.get("ip")).collect())
}

/// Save advanced options for a torrent
pub async fn save_advanced_options(
    pool: &SqlitePool,
    info_hash: &str,
    seed_ratio_limit: Option<f64>,
    max_connections: Option<usize>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO torrent_advanced_options (info_hash, seed_ratio_limit, max_connections)
        VALUES (?, ?, ?)
        ON CONFLICT(info_hash) DO UPDATE SET
            seed_ratio_limit = excluded.seed_ratio_limit,
            max_connections = excluded.max_connections
        "#,
    )
    .bind(info_hash)
    .bind(seed_ratio_limit)
    .bind(max_connections.map(|c| c as i64))
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to save advanced options: {}", e)))?;

    Ok(())
}

/// Save torrent files
async fn save_torrent_files(
    pool: &SqlitePool,
    info_hash: &str,
    files: &[TorrentFile],
) -> Result<(), AppError> {
    // Delete existing files
    sqlx::query("DELETE FROM torrent_files WHERE info_hash = ?")
        .bind(info_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete torrent files: {}", e)))?;

    // Insert new files
    for file in files {
        sqlx::query(
            "INSERT INTO torrent_files (info_hash, path, size) VALUES (?, ?, ?)"
        )
        .bind(info_hash)
        .bind(file.path.to_string_lossy().to_string())
        .bind(file.size as i64)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save torrent file: {}", e)))?;
    }

    Ok(())
}

/// Save torrent tags
async fn save_torrent_tags(
    pool: &SqlitePool,
    info_hash: &str,
    tags: &[String],
) -> Result<(), AppError> {
    // Delete existing tags
    sqlx::query("DELETE FROM torrent_tags WHERE info_hash = ?")
        .bind(info_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete torrent tags: {}", e)))?;

    // Insert new tags
    for tag in tags {
        sqlx::query(
            "INSERT INTO torrent_tags (info_hash, tag) VALUES (?, ?)"
        )
        .bind(info_hash)
        .bind(tag)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save torrent tag: {}", e)))?;
    }

    Ok(())
}

/// Save bandwidth limit
async fn save_bandwidth_limit(
    pool: &SqlitePool,
    info_hash: &str,
    limit: &BandwidthLimit,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO torrent_bandwidth_limits (info_hash, download_limit, upload_limit, enabled)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(info_hash) DO UPDATE SET
            download_limit = excluded.download_limit,
            upload_limit = excluded.upload_limit,
            enabled = excluded.enabled
        "#,
    )
    .bind(info_hash)
    .bind(limit.download_limit.map(|l| l as i64))
    .bind(limit.upload_limit.map(|l| l as i64))
    .bind(limit.enabled)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to save bandwidth limit: {}", e)))?;

    Ok(())
}

/// Save schedule
async fn save_schedule(
    pool: &SqlitePool,
    info_hash: &str,
    schedule: &TorrentSchedule,
) -> Result<(), AppError> {
    let days_json = if schedule.days_of_week.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&schedule.days_of_week).unwrap_or_default())
    };

    sqlx::query(
        r#"
        INSERT INTO torrent_schedules (info_hash, start_time, end_time, days_of_week, enabled)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(info_hash) DO UPDATE SET
            start_time = excluded.start_time,
            end_time = excluded.end_time,
            days_of_week = excluded.days_of_week,
            enabled = excluded.enabled
        "#,
    )
    .bind(info_hash)
    .bind(&schedule.start_time)
    .bind(&schedule.end_time)
    .bind(days_json)
    .bind(schedule.enabled)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to save schedule: {}", e)))?;

    Ok(())
}

/// Load torrent from database
pub async fn load_torrent(
    pool: &SqlitePool,
    info_hash: &str,
) -> Result<Option<(TorrentRow, Vec<TorrentFileRow>, Vec<String>, TorrentBandwidthRow, TorrentScheduleRow)>, AppError> {
    // Load main torrent data
    let torrent = sqlx::query_as::<_, TorrentRow>(
        "SELECT * FROM torrents WHERE info_hash = ?"
    )
    .bind(info_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to load torrent: {}", e)))?;

    if torrent.is_none() {
        return Ok(None);
    }
    let torrent = torrent.unwrap();

    // Load files
    let files = sqlx::query_as::<_, TorrentFileRow>(
        "SELECT id, info_hash, path, size FROM torrent_files WHERE info_hash = ?"
    )
    .bind(info_hash)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to load torrent files: {}", e)))?;

    // Load tags
    let tags: Vec<String> = sqlx::query("SELECT tag FROM torrent_tags WHERE info_hash = ?")
        .bind(info_hash)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to load torrent tags: {}", e)))?
        .iter()
        .map(|row| row.get("tag"))
        .collect();

    // Load bandwidth limit
    let bandwidth = sqlx::query_as::<_, TorrentBandwidthRow>(
        "SELECT * FROM torrent_bandwidth_limits WHERE info_hash = ?"
    )
    .bind(info_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to load bandwidth limit: {}", e)))?
    .unwrap_or(TorrentBandwidthRow {
        info_hash: info_hash.to_string(),
        download_limit: None,
        upload_limit: None,
        enabled: false,
    });

    // Load schedule
    let schedule = sqlx::query_as::<_, TorrentScheduleRow>(
        "SELECT * FROM torrent_schedules WHERE info_hash = ?"
    )
    .bind(info_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to load schedule: {}", e)))?
    .unwrap_or(TorrentScheduleRow {
        info_hash: info_hash.to_string(),
        start_time: None,
        end_time: None,
        days_of_week: None,
        enabled: false,
    });

    Ok(Some((torrent, files, tags, bandwidth, schedule)))
}

/// Load all torrents from database
pub async fn load_all_torrents(
    pool: &SqlitePool,
) -> Result<Vec<String>, AppError> {
    let hashes: Vec<String> = sqlx::query("SELECT info_hash FROM torrents ORDER BY added_time DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to load torrents: {}", e)))?
        .iter()
        .map(|row| row.get("info_hash"))
        .collect();

    Ok(hashes)
}

/// Delete torrent from database
pub async fn delete_torrent(
    pool: &SqlitePool,
    info_hash: &str,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM torrents WHERE info_hash = ?")
        .bind(info_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete torrent: {}", e)))?;

    // Cascading deletes will handle related tables
    Ok(())
}

/// Update torrent statistics
pub async fn update_torrent_stats(
    pool: &SqlitePool,
    info_hash: &str,
    stats: &TorrentStats,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE torrents SET
            downloaded_size = ?,
            uploaded_size = ?,
            download_rate = ?,
            upload_rate = ?,
            peers = ?,
            seeders = ?,
            progress = ?,
            eta = ?
        WHERE info_hash = ?
        "#,
    )
    .bind(stats.downloaded as i64)
    .bind(stats.uploaded as i64)
    .bind(stats.download_rate as i64)
    .bind(stats.upload_rate as i64)
    .bind(stats.peers as i32)
    .bind(stats.seeders as i32)
    .bind(stats.progress)
    .bind(stats.eta.map(|e| e as i64))
    .bind(info_hash)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to update torrent stats: {}", e)))?;

    Ok(())
}
