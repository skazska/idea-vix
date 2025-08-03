-- Add down migration script here
DROP TABLE `session`;
DROP INDEX `idx_session_address`;
DROP TABLE `package_rel`;
DROP TABLE `board_rel`;