#!/usr/bin/env coffee

> @3-/kv:KV

key = 'uid'
uid = await KV.incr key

min = 47
if uid < min
  await KV.set key, min
  console.log 'init '+key, min

process.exit()
