-- Add up migration script here
CREATE TABLE `board`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` TEXT NOT NULL,
	`description` TEXT DEFAULT NULL,
	`icon` TEXT DEFAULT NULL
);
