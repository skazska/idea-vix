-- Add up migration script here
CREATE TABLE `package`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` VARCHAR(100) NOT NULL,
    `slug` VARCHAR(100) NOT NULL,
	`description` VARCHAR(500) DEFAULT NULL,
	`icon` VARCHAR(255) DEFAULT NULL,
	`is_public` BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create index for slug lookups (important for text-based references)
CREATE INDEX idx_package_slug ON package(`slug`);

CREATE TABLE `package_shape` (
    package_id INTEGER NOT NULL,
    shape_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (package_id, shape_id),
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE CASCADE,
    FOREIGN KEY (shape_id) REFERENCES shape(id) ON DELETE CASCADE
);

CREATE TABLE `package_line` (
    package_id INTEGER NOT NULL,
    line_id INTEGER NOT NULL,
    PRIMARY KEY (package_id, line_id),
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE CASCADE,
    FOREIGN KEY (line_id) REFERENCES line(id) ON DELETE CASCADE
);

CREATE TABLE `package_rule` (
    package_id INTEGER NOT NULL,
    rule_id INTEGER NOT NULL,
    PRIMARY KEY (package_id, rule_id),
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE CASCADE,
    FOREIGN KEY (rule_id) REFERENCES rule(id) ON DELETE CASCADE
);

