CREATE FUNCTION `authUidMailNew`(`hostId` BIGINT UNSIGNED,`mail` VARBINARY(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE mailId BIGINT UNSIGNED;
  DECLARE uid BIGINT UNSIGNED;
  SELECT authMailNew(mail) INTO mailId;
  SELECT id INTO uid FROM authUidMail t WHERE t.authMailId=mailId AND t.hostId=hostId;
  IF uid IS NULL THEN
    INSERT INTO uidHost (hostId) VALUES (hostId);
    SELECT LAST_INSERT_ID() INTO uid;
    INSERT INTO authUidMail (id,hostId,authMailId) VALUES (uid,hostId,mailId);
  END IF;
  RETURN uid;
END ;;