CREATE FUNCTION `authMailNew`(mail VARBINARY(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE h BIGINT UNSIGNED;
  DECLARE mid BIGINT UNSIGNED;
  CALL authSplitMail(mail,@u,@h);
  SELECT authMailHostId(@h) INTO h;
  SELECT id INTO mid FROM authMail WHERE usr=@u AND authHostId=h;
  IF mid IS NULL THEN
    INSERT INTO authMail (usr,authHostId) VALUES (@u,h);
  RETURN LAST_INSERT_ID();
  END IF;
return mid;
END ;;