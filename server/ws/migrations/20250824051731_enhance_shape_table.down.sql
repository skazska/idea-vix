-- Migration: rollback shape table enhancements
-- Remove the sample data first
DELETE FROM shape WHERE slug IN ('rectangle', 'circle', 'diamond');

-- Drop the index
DROP INDEX IF EXISTS idx_shape_slug;

-- Remove the new columns from package_shape
ALTER TABLE package_shape DROP COLUMN created_at;

-- Remove the new columns from shape table
-- Note: SQLite doesn't support dropping columns directly in older versions
-- We need to recreate the table without the new columns

-- Create backup table with original structure
CREATE TABLE shape_backup AS SELECT id, name FROM shape;

-- Drop the current table
DROP TABLE shape;

-- Recreate original table structure
CREATE TABLE "shape" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR(100) NOT NULL
);

-- Restore original data
INSERT INTO shape (id, name) SELECT id, name FROM shape_backup;

-- Drop backup table
DROP TABLE shape_backup;
