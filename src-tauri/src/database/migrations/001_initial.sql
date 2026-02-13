-- Initial database schema for AFK-Dunld Download Manager
-- This file mirrors the schema defined in src-tauri/src/database/db.rs

-- Downloads table
CREATE TABLE IF NOT EXISTS downloads (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    final_url TEXT,
    file_name TEXT NOT NULL,
    save_path TEXT NOT NULL,
    total_size INTEGER,
    downloaded_size INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Queued',
    segments INTEGER NOT NULL DEFAULT 4,
    supports_range BOOLEAN NOT NULL DEFAULT FALSE,
    content_type TEXT,
    etag TEXT,
    expected_checksum TEXT,
    actual_checksum TEXT,
    checksum_algorithm TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT,
    priority INTEGER NOT NULL DEFAULT 100,
    category TEXT,
    segment_progress TEXT
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_downloads_status
    ON downloads(status);
CREATE INDEX IF NOT EXISTS idx_downloads_created
    ON downloads(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_downloads_category
    ON downloads(category);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
