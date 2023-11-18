use conn::fred::{
  interfaces::{RedisResult, SortedSetsInterface},
  prelude::FunctionInterface,
  types::FromRedis,
};

use crate::{lua, K};

pub async fn host<T: FromRedis, C: SortedSetsInterface + Sync>(p: &C, key: &str) -> RedisResult<T> {
  p.zscore(K::HOST_ID, xstr::word_reverse(key, ".")).await
}

pub fn reverse_mail(mail: &str) -> String {
  xstr::word_reverse(mail, "@.")
}

pub async fn mail<T: FromRedis, C: SortedSetsInterface + Sync>(p: &C, key: &str) -> RedisResult<T> {
  p.zscore(K::MAIL_ID, reverse_mail(key)).await
}

pub async fn mail_new_if_not_exist<R: FromRedis, C: Sync + FunctionInterface>(
  p: &C,
  key: &str,
) -> RedisResult<R> {
  p.fcall(lua::ZSET_ID, &[K::MAIL_ID], [reverse_mail(key)])
    .await
}
