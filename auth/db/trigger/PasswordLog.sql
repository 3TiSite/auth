CREATE TRIGGER `authPasswordLog` BEFORE UPDATE ON `authPasswd` FOR EACH ROW BEGIN
  INSERT IGNORE INTO authPasswdLog (hostId,uid,hash,ts)
  VALUES (OLD.hostId,OLD.uid,OLD.hash,OLD.ts);
END ;;