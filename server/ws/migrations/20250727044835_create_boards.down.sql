-- Add down migration script here
DROP INDEX IF EXISTS "idx_board_slug";    
DROP TABLE `board`;
DROP TABLE `board_package`;
DROP TABLE `board_shape`;
DROP TABLE `board_line`;
DROP TABLE `board_rule`;
DROP TABLE IF EXISTS `package_layout`;
