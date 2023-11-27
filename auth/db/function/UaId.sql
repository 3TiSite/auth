CREATE FUNCTION `authUaId`(w SMALLINT UNSIGNED,h SMALLINT UNSIGNED,pixelRatio TINYINT UNSIGNED,zone SMALLINT,cpu SMALLINT UNSIGNED,os VARCHAR(255),osMajor SMALLINT UNSIGNED,osMinor SMALLINT UNSIGNED,browser VARCHAR(255),browserMajor SMALLINT UNSIGNED,browserMinor SMALLINT UNSIGNED,gpu VARCHAR(255),lang VARCHAR(255),arch VARCHAR(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE authHardwareId BIGINT UNSIGNED;
  DECLARE authOsVerId BIGINT UNSIGNED;
  DECLARE authBrowserVerId BIGINT UNSIGNED;
  DECLARE authLangId BIGINT UNSIGNED;
  DECLARE r BIGINT UNSIGNED;
  SELECT authHardwareId(w,h,pixelRatio,cpu,gpu,arch)  INTO authHardwareId;
  SELECT authOsVerId(os,osMajor,osMinor) INTO authOsVerId;
  SELECT authBrowserVerId(browser,browserMajor,browserMinor) INTO authBrowserVerId;
  SELECT authLangId(lang) INTO authLangId;
  SELECT id INTO r FROM authUa t
  WHERE t.authHardwareId=authHardwareId
  AND t.zone=zone
  AND t.authOsVerId=authOsVerId
  AND t.authBrowserVerId=authBrowserVerId
  AND t.authLangId=authLangId;
  IF r IS NULL THEN
    INSERT INTO authUa(authHardwareId,zone,authOsVerId,authBrowserVerId,authLangId) VALUES (authHardwareId,zone,authOsVerId,authBrowserVerId,authLangId);
    RETURN LAST_INSERT_ID();
  END IF;
RETURN r;
END ;;