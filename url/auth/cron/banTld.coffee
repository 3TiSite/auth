#!/usr/bin/env coffee

> @3-/req/reqJson.js
  @3-/write
  @3-/kv:KV
  ./conf > PWD
  path > join
  tld-extract:tld
  punycode

PROXY='https://gh-proxy.com/'
TS = 'banTldTs'
DAY = 86400
WEEK = DAY * 7

< main = =>
  ts = await KV.get TS
  if ts # and false
    ts = parseInt(ts,36)
    diff = WEEK - ((new Date/1000) - ts)
    if diff > 0
      console.log 'banTld will update after', Math.round(diff*1e3/DAY)/1e3,'days'
      return

  url = PROXY+'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json'
  {t,domains} = await reqJson url

  li = new Set
  for [host,o] from Object.entries domains
    {lastseen} = o
    if (t - lastseen)/86400 < 365
      host = tld 'http://'+host, allowUnknownTLD:true
      host = punycode.toUnicode(host.domain.trim().toLowerCase()) .split '.'
      if host.length
        host = host.reverse().join '.'
        li.add host

  li = [...li]
  li.sort()
  p = KV.pipeline()
  p.sadd 'banTld', ...li
  p.set TS, (parseInt(new Date()/1000)).toString(36)
  p.exec()


if process.argv[1] == decodeURI(new URL(import.meta.url).pathname)
  await main()
  process.exit()
