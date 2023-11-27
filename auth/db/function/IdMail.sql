CREATE FUNCTION `authIdMail`(mailId BIGINT UNSIGNED) RETURNS varbinary(255)
BEGIN
return (select CONCAT(authMail.usr,'@',val) from authMail,authMailHost WHERE authMail.idmailId AND authMailHost.id=authHostId);
END ;;