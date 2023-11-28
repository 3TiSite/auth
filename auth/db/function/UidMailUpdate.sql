CREATE FUNCTION `authUidMailUpdate`(`uid` BIGINT UNSIGNED,`mail` VARBINARY(255)) RETURNS TINYINT
BEGIN
  DECLARE mailId BIGINT UNSIGNED;
  DECLARE uid BIGINT UNSIGNED;
  SELECT authMailNew(mail) INTO mailId;
  SELECT id INTO uid FROM authUidMail t WHERE t.id=uid AND t.authMailId=mailId;
  IF uid IS NOT NULL THEN
    # new mail used
    RETURN -1;
  END IF;
  UPDATE authUidMail SET authMailId=mailId WHERE id=uid;
  RETURN 0;
END ;;