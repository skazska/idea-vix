-- Add down migration script here
DROP INDEX IF EXISTS "idx_package_slug";    
DROP TABLE `package`;
DROP TABLE `package_shape`;
DROP TABLE `package_line`;
DROP TABLE `package_rule`;

-- Remove package_layout table
DROP TABLE IF EXISTS `package_layout`;
