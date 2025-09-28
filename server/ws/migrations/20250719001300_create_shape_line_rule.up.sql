-- Migration: create shape, line, and rule tables with enhanced shape structure
CREATE TABLE "shape" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "description" VARCHAR(500),
    "definition" TEXT NOT NULL, -- JSON as TEXT in SQLite
    "version" VARCHAR(30) DEFAULT '1.0', -- Versioning for shape definitions
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index for slug lookups (important for text-based references)
CREATE UNIQUE INDEX idx_shape_slug ON shape(`slug`);

CREATE TABLE "line" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "description" VARCHAR(500),
    "definition" TEXT NOT NULL, -- JSON as TEXT in SQLite
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index for slug lookups (important for text-based references)
CREATE UNIQUE INDEX idx_line_slug ON line(`slug`);

CREATE TABLE "rule" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "description" VARCHAR(500),
    "definition" TEXT NOT NULL, -- JSON as TEXT in SQLite
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index for slug lookups (important for text-based references)
CREATE UNIQUE INDEX idx_rule_slug ON rule(`slug`);

CREATE TABLE "layout" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "description" VARCHAR(500),
    "definition" TEXT NOT NULL, -- JSON as TEXT in SQLite
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index for slug lookups (important for text-based references)
CREATE UNIQUE INDEX idx_layout_slug ON layout(`slug`);
