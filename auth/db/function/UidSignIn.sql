CREATE FUNCTION `authUidSignIn`(uid BIGINT UNSIGNED,clientId BIGINT UNSIGNED,ip VARBINARY(16),authUaId BIGINT UNSIGNED) RETURNS TINYINT
BEGIN
  DECLARE sid BIGINT UNSIGNED;
  INSERT INTO authUidSignIn (clientId,uid,ip,authUaId) VALUES (clientId,uid,ip,authUaId);
  SELECT LAST_INSERT_ID() INTO sid;
  INSERT INTO authUidClient (uid,client,state,lastSignInId) VALUES (uid,clientId,1,sid) ON DUPLICATE KEY UPDATE state=1,lastSignInId=sid;
  RETURN NULL;
END ;;