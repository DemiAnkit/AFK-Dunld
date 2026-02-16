-- Migration to add download categories

-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    icon TEXT,
    save_path TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Add category_id to downloads table
ALTER TABLE downloads ADD COLUMN category_id TEXT REFERENCES categories(id);

-- Create index on category_id
CREATE INDEX IF NOT EXISTS idx_downloads_category ON downloads(category_id);

-- Insert default categories
INSERT OR IGNORE INTO categories (id, name, color, icon, save_path, created_at, updated_at)
VALUES 
    ('default', 'Default', '#6B7280', 'folder', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('documents', 'Documents', '#3B82F6', 'file-text', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('videos', 'Videos', '#EF4444', 'video', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('music', 'Music', '#8B5CF6', 'music', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('images', 'Images', '#10B981', 'image', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('software', 'Software', '#F59E0B', 'package', NULL, strftime('%s', 'now'), strftime('%s', 'now')),
    ('compressed', 'Archives', '#6366F1', 'archive', NULL, strftime('%s', 'now'), strftime('%s', 'now'));
