use r::{
  fred::interfaces::{RedisResult, SetsInterface},
  KV,
};

use crate::K;

pub async fn is(tld: impl AsRef<str>) -> RedisResult<bool> {
  let tld = xstr::reverse(tld.as_ref());
  KV.sismember(K::BAN_TLD, tld).await
}
