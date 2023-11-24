function mailUidPasswd(KEYS, ARGS)
	-- flags no-writes
	local mail_uid, passwd = unpack(KEYS)
	local mail_id = unpack(ARGS)
	local uid = ZSCORE(mail_uid, mail_id)
	if uid then
		uid = intBin(uid)
		passwd = HGET(passwd, uid)
		if passwd then
			return { uid, passwd }
		end
		return { uid }
	end
end

function mailUidPasswdSet(KEYS, ARGS)
	local mail_uid, passwd_key = unpack(KEYS)
	local mail_id, passwd = unpack(ARGS)
	local uid = ZSCORE(mail_uid, mail_id)
	if uid then
		uid = intBin(uid)
		HSET(passwd_key, uid, passwd)
		return uid
	end
end

function accountNewUidPasswd(KEYS, ARGS)
	local zset, gid, mail_uid_key, uid_account_key, uid_host_key, passwd_key = unpack(KEYS)
	local mail_id_bin, account, host_bin, passwd = unpack(ARGS)
	local uid = zsetGid(zset, gid, mail_id_bin)

	local uid_bin = intBin(uid)
	HSET(uid_account_key, uid_bin, account)
	HSETNX(uid_host_key, uid_bin, host_bin)
	ZADD_NX(mail_uid_key, TS(), uid_bin)
	local exist = HSETNX(passwd_key, uid_bin, passwd)
	if exist == 0 then
		return { uid_bin, HGET(passwd_key, uid_bin) }
	end
	return { uid_bin }
end

function uidSetMail(KEYS, ARGS)
	local uid_account_key, host_mail_uid_key, new_mail_uid_key, old_mail_uid_key = unpack(KEYS)
	local uid_bin, new_mail, new_mail_id_bin, old_mail_id_bin = unpack(ARGS)
	local uid = binInt(uid_bin)
	HSET(uid_account_key, uid_bin, new_mail)
	ZADD(host_mail_uid_key, uid, new_mail_id_bin)
	ZADD_NX(new_mail_uid_key, TS(), uid_bin)
	if old_mail_uid_key then
		ZREM(old_mail_uid_key, uid_bin)
		ZREM(host_mail_uid_key, old_mail_id_bin)
	end
end
