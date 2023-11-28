CREATE TRIGGER `authUidMailUpdate` BEFORE UPDATE ON `authUidMail` FOR EACH ROW BEGIN
  INSERT IGNORE INTO authUidMailLog (uid,authMailId,hostId)
  VALUES (OLD.id,OLD.authMailId,OLD.hostId);
END ;;