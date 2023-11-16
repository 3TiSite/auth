use conn::{
  fred::interfaces::{RedisResult, SortedSetsInterface},
  KV,
};

use crate::K;

pub async fn bin(host_bin: impl AsRef<[u8]>, uid: u64) -> RedisResult<Option<Vec<u8>>> {
  let mut mail_id: Vec<Vec<u8>> = KV
    .zrangebyscore(
      K::host_mail_uid(host_bin.as_ref()),
      uid as i64,
      uid as i64,
      false,
      Some((0, 1)),
    )
    .await?;

  Ok(if mail_id.is_empty() {
    None
  } else {
    Some(mail_id.remove(0))
  })
}
