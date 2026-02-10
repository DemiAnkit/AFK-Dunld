-- Downloads table
CREATE TABLE IF NOT EXISTS downloads (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    file_name TEXT NOT NULL,
    save_path TEXT NOT NULL,
    total_size INTEGER,
    downloaded_size INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',
    segments INTEGER DEFAULT 4,
    retries INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    supports_range BOOLEAN DEFAULT 1,
    content_type TEXT,
    etag TEXT,
    expected_checksum TEXT,
    checksum_type TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT,
    priority INTEGER DEFAULT 5,
    speed_limit INTEGER,
    category TEXT DEFAULT 'general'
);

-- Create index for faster queries
CREATE INDEX IF NOT EXISTS idx_downloads_status ON downloads(status);
CREATE INDEX IF NOT EXISTS idx_downloads_created_at ON downloads(created_at);
CREATE INDEX IF NOT EXISTS idx_downloads_category ON downloads(category);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES 
    ('download_path', '', datetime('now')),
    ('max_concurrent_downloads', '3', datetime('now')),
    ('default_segments', '4', datetime('now')),
    ('speed_limit', '0', datetime('now')),
    ('theme', 'system', datetime('now')),
    ('start_with_system', 'false', datetime('now')),
    ('show_notifications', 'true', datetime('now')),
    ('monitor_clipboard', 'true', datetime('now')),
    ('auto_start_downloads', 'false', datetime('now')),
    ('default_category', 'general', datetime('now'));

-- Queue table for scheduled downloads
CREATE TABLE IF NOT EXISTS download_queue (
    id TEXT PRIMARY KEY,
    download_id TEXT NOT NULL UNIQUE,
    queue_position INTEGER NOT NULL,
    scheduled_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_queue_position ON download_queue(queue_position);

-- Download history/log table
CREATE TABLE IF NOT EXISTS download_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_logs_download_id ON download_logs(download_id);
