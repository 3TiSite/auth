use conn::fred::{
  interfaces::{RedisResult, SortedSetsInterface},
  types::FromRedis,
};

use crate::K;

pub async fn host<T: FromRedis, R: SortedSetsInterface + Sync>(p: &R, key: &str) -> RedisResult<T> {
  p.zscore(K::HOST_ID, xstr::word_reverse(key, ".")).await
}

pub fn reverse_mail(mail: &str) -> String {
  xstr::word_reverse(mail, "@.")
}

pub async fn mail<T: FromRedis, R: SortedSetsInterface + Sync>(p: &R, key: &str) -> RedisResult<T> {
  p.zscore(K::MAIL_ID, reverse_mail(key)).await
}
