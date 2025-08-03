-- Add up migration script here
CREATE TABLE `session`(
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
	`address` VARCHAR(255) NOT NULL,
    `code` VARCHAR(255) NOT NULL,
    `sent_at` DATETIME NOT NULL,
    `expires_at` DATETIME NOT NULL
);

CREATE INDEX `idx_session_address` ON `session` (`address`, `code`);

CREATE TABLE `package_rel` (
    package_id INTEGER NOT NULL,
    address VARCHAR(255) NOT NULL,
    PRIMARY KEY (package_id, address),
    FOREIGN KEY (package_id) REFERENCES package(id) ON DELETE CASCADE
);

CREATE TABLE `board_rel` (
    board_id INTEGER NOT NULL,
    address VARCHAR(255) NOT NULL,
    PRIMARY KEY (board_id, address),
    FOREIGN KEY (board_id) REFERENCES board(id) ON DELETE CASCADE
);
