CREATE PROCEDURE `tokenTurn`(IN `uid` BIGINT UNSIGNED,IN `id` BIGINT UNSIGNED,IN `enable` TINYINT)
BEGIN
    DECLARE r TINYINT UNSIGNED;
    UPDATE token SET token.enable=enable WHERE token.id=id AND token.uid=uid;
    SET r=ROW_COUNT();
    IF r > 0 THEN
       IF enable > 0 THEN
        SELECT sk,day FROM token WHERE token.id=id;
       ELSE
        SELECT 0 AS sk,0 AS day;
       END IF;
    END IF;
END ;;