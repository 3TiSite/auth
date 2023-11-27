#!/usr/bin/env coffee

> @3-/kv:KV
  @3-/reverse
  @3-/dbq

fcall = (func, keys, vals)=>
  keys.unshift keys.length
  KV.fcall(func, keys, vals)

main = ()=>
  host = process.argv[2]
  host = host.trim().toLowerCase()
  if not host
    return

  rhost = reverse(host)
  id = await fcall(
    'zsetId'
    [
      'hostId'
    ]
    [
      rhost
    ]
  )
  await dbq(
    "INSERT IGNORE INTO authHost (id,val,ts) VALUES (?,?,UNIX_TIMESTAMP())"
    id
    host
  )
  console.log host, id
  return

await main()
process.exit()
