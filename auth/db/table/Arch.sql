CREATE TABLE `authArch` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `val` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `val` (`val`)
);