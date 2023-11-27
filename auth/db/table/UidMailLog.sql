CREATE TABLE `authUidMailLog` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `uid` BIGINT UNSIGNED NOT NULL,
  `authMailId` BIGINT UNSIGNED NOT NULL,
  `authHostId` BIGINT UNSIGNED NOT NULL,
  `cts` BIGINT NOT NULL,
  `dts` BIGINT UNSIGNED NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`,`authHostId`,`cts`)
);