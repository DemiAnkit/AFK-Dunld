-- Torrent support schema
-- Migration 003: Add tables for torrent metadata and tracking

-- Torrents table - stores torrent metadata
CREATE TABLE IF NOT EXISTS torrents (
    info_hash TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    total_size INTEGER NOT NULL,
    piece_length INTEGER NOT NULL,
    num_pieces INTEGER NOT NULL,
    save_path TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 1,
    category TEXT,
    added_time TEXT NOT NULL,
    completed_time TEXT,
    state TEXT NOT NULL DEFAULT 'Downloading',
    downloaded_size INTEGER NOT NULL DEFAULT 0,
    uploaded_size INTEGER NOT NULL DEFAULT 0,
    download_rate INTEGER NOT NULL DEFAULT 0,
    upload_rate INTEGER NOT NULL DEFAULT 0,
    peers INTEGER NOT NULL DEFAULT 0,
    seeders INTEGER NOT NULL DEFAULT 0,
    progress REAL NOT NULL DEFAULT 0.0,
    eta INTEGER
);

-- Torrent files table - stores individual files within torrents
CREATE TABLE IF NOT EXISTS torrent_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    info_hash TEXT NOT NULL,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    FOREIGN KEY (info_hash) REFERENCES torrents(info_hash) ON DELETE CASCADE
);

-- Torrent tags table - many-to-many relationship for tags
CREATE TABLE IF NOT EXISTS torrent_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    info_hash TEXT NOT NULL,
    tag TEXT NOT NULL,
    UNIQUE(info_hash, tag),
    FOREIGN KEY (info_hash) REFERENCES torrents(info_hash) ON DELETE CASCADE
);

-- Torrent bandwidth limits table
CREATE TABLE IF NOT EXISTS torrent_bandwidth_limits (
    info_hash TEXT PRIMARY KEY,
    download_limit INTEGER,
    upload_limit INTEGER,
    enabled BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (info_hash) REFERENCES torrents(info_hash) ON DELETE CASCADE
);

-- Torrent schedules table
CREATE TABLE IF NOT EXISTS torrent_schedules (
    info_hash TEXT PRIMARY KEY,
    start_time TEXT,
    end_time TEXT,
    days_of_week TEXT,
    enabled BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (info_hash) REFERENCES torrents(info_hash) ON DELETE CASCADE
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_torrents_state ON torrents(state);
CREATE INDEX IF NOT EXISTS idx_torrents_priority ON torrents(priority DESC);
CREATE INDEX IF NOT EXISTS idx_torrents_category ON torrents(category);
CREATE INDEX IF NOT EXISTS idx_torrents_added ON torrents(added_time DESC);
CREATE INDEX IF NOT EXISTS idx_torrent_files_hash ON torrent_files(info_hash);
CREATE INDEX IF NOT EXISTS idx_torrent_tags_hash ON torrent_tags(info_hash);
CREATE INDEX IF NOT EXISTS idx_torrent_tags_tag ON torrent_tags(tag);
