SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

DROP DATABASE IF EXISTS `url_shorter`;
CREATE DATABASE `url_shorter`;
USE `url_shorter`;

DROP TABLE IF EXISTS `url_info`;
CREATE TABLE `url_info`  (
  `id` int(32) NOT NULL AUTO_INCREMENT,
  `long_url` varchar(511) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `mur_hash_code` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `insert_at` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `latest_visit_at` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `visit_count` int(64) NOT NULL,
  `is_deleted` tinyint(1) NOT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = MyISAM AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_bin ROW_FORMAT = DYNAMIC;

SET FOREIGN_KEY_CHECKS = 1;
