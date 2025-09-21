-- Add down migration script here
DROP TABLE `session`;
DROP INDEX `idx_session_address_code`;
DROP TABLE `package_access_roles`;
DROP TABLE `board_access_roles`;