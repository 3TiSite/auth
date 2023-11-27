CREATE TABLE `authMail` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `usr` varbinary(255) NOT NULL,
  `authHostId` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unqiue` (`authHostId`,`usr`)
);