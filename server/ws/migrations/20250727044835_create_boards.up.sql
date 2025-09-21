-- Add up migration script here
CREATE TABLE `board`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` VARCHAR(100) NOT NULL,
    `slug` VARCHAR(100) NOT NULL,
	`description` VARCHAR(500) DEFAULT NULL,
	`icon` VARCHAR(255) DEFAULT NULL,
	`is_public` BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create index for slug lookups (important for text-based references)
CREATE UNIQUE INDEX idx_board_slug ON board(`slug`);

CREATE TABLE `board_package` (
    board_id INTEGER NOT NULL,
    package_id INTEGER NOT NULL,
    PRIMARY KEY (board_id, package_id),
    FOREIGN KEY (board_id) REFERENCES board(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE CASCADE
);

CREATE TABLE `board_shape` (
    board_id INTEGER NOT NULL,
    shape_id INTEGER NOT NULL,
    package_id INTEGER,
    name VARCHAR(100),
    PRIMARY KEY (board_id, shape_id, package_id),
    FOREIGN KEY (board_id) REFERENCES board(id) ON DELETE CASCADE,
    FOREIGN KEY (shape_id) REFERENCES shape(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE SET NULL
);

CREATE TABLE `board_line` (
    board_id INTEGER NOT NULL,
    line_id INTEGER NOT NULL,
    package_id INTEGER,
    name VARCHAR(100),
    PRIMARY KEY (board_id, line_id, package_id),
    FOREIGN KEY (board_id) REFERENCES board(id) ON DELETE CASCADE,
    FOREIGN KEY (line_id) REFERENCES line(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE SET NULL
);

CREATE TABLE `board_rule` (
    board_id INTEGER NOT NULL,
    rule_id INTEGER NOT NULL,
    package_id INTEGER,
    PRIMARY KEY (board_id, rule_id, package_id),
    FOREIGN KEY (board_id) REFERENCES board(id) ON DELETE CASCADE,
    FOREIGN KEY (rule_id) REFERENCES rule(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE SET NULL
);
