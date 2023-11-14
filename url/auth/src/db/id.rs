use conn::fred::interfaces::{RedisResult, SortedSetsInterface};

use crate::K;

pub async fn host<R: SortedSetsInterface + Sync>(p: &R, key: &str) -> RedisResult<u64> {
  p.zscore(K::HOST_ID, xstr::word_reverse(key, ".")).await
}

pub fn reverse_mail(mail: &str) -> String {
  xstr::word_reverse(mail, "@.")
}

pub async fn mail<R: SortedSetsInterface + Sync>(p: &R, key: &str) -> RedisResult<u64> {
  p.zscore(K::MAIL_ID, reverse_mail(key)).await
}
