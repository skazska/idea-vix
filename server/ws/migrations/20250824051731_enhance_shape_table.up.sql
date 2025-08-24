-- Migration: enhance shape table with semantic identifiers and definition storage
-- SQLite doesn't support adding UNIQUE columns directly, so we need to recreate the table

-- Create new table with enhanced structure
CREATE TABLE shape_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE,
    description VARCHAR(500),
    definition TEXT, -- JSON as TEXT in SQLite
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Copy existing data from old table
INSERT INTO shape_new (id, name) SELECT id, name FROM shape;

-- Drop old table
DROP TABLE shape;

-- Rename new table to original name
ALTER TABLE shape_new RENAME TO shape;

-- Create index for slug lookups (important for text-based references)
CREATE INDEX idx_shape_slug ON shape(slug);

-- Update package_shape table to include creation timestamp
ALTER TABLE package_shape ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;

-- Add some sample shapes with semantic identifiers for testing
INSERT INTO shape (name, slug, description, definition) VALUES 
    ('Rectangle', 'rectangle', 'Basic rectangular shape', '{"background":{"border":{"path":"M 0 0 L 100 0 L 100 50 L 0 50 Z","stroke":{"width":2,"color":"#333"},"fill":{"color":"#f0f0f0"}}}}'),
    ('Circle', 'circle', 'Basic circular shape', '{"background":{"border":{"path":"M 50 25 A 25 25 0 1 1 49.99 25","stroke":{"width":2,"color":"#333"},"fill":{"color":"#f0f0f0"}}}}'),
    ('Diamond', 'diamond', 'Diamond decision shape', '{"background":{"border":{"path":"M 50 0 L 100 25 L 50 50 L 0 25 Z","stroke":{"width":2,"color":"#333"},"fill":{"color":"#f0f0f0"}}}}');
