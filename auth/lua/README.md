# zsetId

用中文回复，写 redis lua 函数 , 代码要尽量简洁优雅，清晰易懂。

定义函数 zsetId(KEYS, ARGS)，实现一下功能

输入的 KEYS 是 zsetId , ARGS 的 一个字符串 mail；
先从 zset zsetId 中获取 mail 的 score
如果有，就返回
如果没有，就 获取 id = zsetId 中最大值 + 1 ，然后记录 mail - id，返回 id

# uid

https://chat.openai.com/share/67cc41c8-0373-4ef4-a3c6-000e08c082c3

用中文回复，写 redis lua 函数 , 代码要尽量简洁优雅，清晰易懂。

定义函数 uid(KEYS, ARGS)，实现一下功能

输入：
KEYS 是 mail{id}user, id 分别是 zset 和 hset
ARGS 的 数值 mail_id

输出：数值 uid

流程：
先去 mail{id}user 查找 mail_id 的 score ，如果有就返回
如果没有，就 hincr id 中的 uid, 然后 zadd 到 mail{id}user 中去（键是 mail_id），最后返回 uid
