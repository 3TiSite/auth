CREATE TABLE `authUidMail` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `authHostId` BIGINT UNSIGNED NOT NULL,
  `uid` BIGINT UNSIGNED NOT NULL,
  `authMailId` BIGINT UNSIGNED NOT NULL,
  `ts` BIGINT UNSIGNED NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique` (`authHostId`,`uid`,`authMailId`)
);