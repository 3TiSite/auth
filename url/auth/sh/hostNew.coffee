#!/usr/bin/env coffee

> @3-/kv:KV

fcall = (func, keys, vals)=>
  keys.unshift keys.length
  KV.fcall(func, keys, vals)

main = =>
  host = process.argv[2]
  host = host.trim().toLowerCase()
  if not host
    return

  console.log host
  dot = '.'
  host = host.split(dot).reverse().join dot
  console.log await fcall(
    'zsetId'
    [
      'hostId'
    ]
    [
      host
    ]
  )
  return


await main()
process.exit()
