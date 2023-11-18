-- local log = function(...)
--   local li = {}
--   for _, v in ipairs({ ... }) do
--     table.insert(li, cjson.encode(v))
--   end
--   redis.log(redis.LOG_WARNING, unpack(li))
-- end

local binInt = function(str)
  local n = 0
  local base = 1
  for i = 1, #str do
    local c = str:sub(i, i)
    n = n + base * c:byte()
    base = base * 256
  end
  return n
end

local intBin = function(n)
  local t = {}
  while n > 0 do
    local r = math.fmod(n, 256)
    table.insert(t, string.char(r))
    n = (n - r) / 256
  end
  return table.concat(t)
end

function zsetId(KEYS, ARGS)
  local zset = KEYS[1]
  local key = ARGS[1]
  local id = redis.call("ZSCORE", zset, key)
  if id then
    return id
  end
  id = (redis.call("ZREVRANGE", zset, 0, 0, "WITHSCORES")[2] or 0) + 1
  redis.call("ZADD", zset, id, key)
  return id
end

function mailUidPasswd(KEYS, ARGS)
  -- flags no-writes
  local mail_uid, passwd = unpack(KEYS)
  local mail_id = unpack(ARGS)
  local uid = redis.call("ZSCORE", mail_uid, mail_id)
  if uid then
    uid = intBin(tonumber(uid))
    passwd = redis.call("HGET", passwd, uid)
    if passwd then
      return { uid, passwd }
    end
    return { uid }
  end
end

function mailUidPasswdSet(KEYS, ARGS)
  local mail_uid, passwd_key = unpack(KEYS)
  local mail_id, passwd = unpack(ARGS)
  local uid = redis.call("ZSCORE", mail_uid, mail_id)
  if uid then
    uid = intBin(tonumber(uid))
    redis.call("HSET", passwd_key, uid, passwd)
    return uid
  end
end

function accountNewUidPasswd(KEYS, ARGS)
  local zset, gid, mail_uid_key, uid_account_key, uid_host_key, passwd_key = unpack(KEYS)
  local mail_id_bin, account, host_bin, passwd = unpack(ARGS)
  local uid = zsetGid(zset, gid, mail_id_bin)

  local uid_bin = intBin(uid)
  redis.call("HSET", uid_account_key, uid_bin, account)
  redis.call("HSETNX", uid_host_key, uid_bin, host_bin)
  redis.call("ZADD", mail_uid_key, "NX", redis.call("time")[1], uid_bin)
  local exist = redis.call("HSETNX", passwd_key, uid_bin, passwd)
  if exist == 0 then
    return { uid_bin, redis.call("HGET", passwd_key, uid_bin) }
  end
  return { uid_bin }
end

function uidSetMail(KEYS, ARGS)
  local uid_account_key, host_mail_uid_key, new_mail_uid_key, old_mail_uid_key = unpack(KEYS)
  local uid_bin, new_mail, new_mail_id_bin, old_mail_id_bin = unpack(ARGS)
  local uid = binInt(uid_bin)
  redis.call("HSET", uid_account_key, uid_bin, new_mail)
  redis.call("ZADD", host_mail_uid_key, uid, new_mail_id_bin)
  redis.call("ZADD", new_mail_uid_key, "NX", redis.call("time")[1], uid_bin)
  if old_mail_uid_key then
    redis.call("ZREM", old_mail_uid_key, uid_bin)
    redis.call("ZREM", host_mail_uid_key, old_mail_id_bin)
  end
end
