use r::fred::{
  interfaces::{RedisResult, SortedSetsInterface},
  types::FromRedis,
};
use xstr::reverse;

use crate::K;

pub async fn _host<T: FromRedis, C: SortedSetsInterface + Sync>(
  p: &C,
  key: impl AsRef<str>,
) -> RedisResult<T> {
  p.zscore(K::HOST_ID, reverse(key)).await
}

pub async fn pipe_id<T: FromRedis, C: SortedSetsInterface + Sync>(
  p: &C,
  key: impl AsRef<str>,
) -> t3::Result<T> {
  tp::host_is_bind(_host(p, key).await?)
}

pub async fn id(host: impl AsRef<str>) -> RedisResult<u64> {
  Ok(_host(&**r::KV, host).await?)
}

pub async fn id_by_header(header: &t3::HeaderMap) -> t3::Result<u64> {
  Ok(by_header(header).await?.1)
}

pub async fn by_header(header: &t3::HeaderMap) -> t3::Result<(String, u64)> {
  let host = t3::origin_tld(header)?;
  let id = pipe_id::<u64, _>(&**r::KV, &host).await?;
  Ok((host, id))
}
