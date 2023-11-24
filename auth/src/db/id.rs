use db::q1;
use r::fred::{
  interfaces::{RedisResult, SortedSetsInterface},
  prelude::FunctionInterface,
  types::FromRedis,
};
use xstr::reverse;

use crate::K;

pub async fn host<T: FromRedis, C: SortedSetsInterface + Sync>(
  p: &C,
  key: impl AsRef<str>,
) -> RedisResult<T> {
  p.zscore(K::HOST_ID, reverse(key)).await
}

pub async fn mail(key: &str) -> db::Result<Option<u64>> {
  Ok(db::q1!("SELECT mailId(?)", key))
}

pub async fn mail_new(key: &str) -> db::Result<u64> {
  Ok(db::q1!("SELECT mailNew(?)", key))
}
