use conn::{
  fred::interfaces::{RedisResult, SortedSetsInterface},
  KV,
};
use intbin::bin_u64;

use crate::{
  db::{id::reverse_mail, uid_mail_id},
  K,
};

pub async fn get(host_bin: impl AsRef<[u8]>, uid: u64) -> RedisResult<String> {
  if let Some(mail_bin) = uid_mail_id::bin(host_bin, uid).await? {
    let mail_id = bin_u64(mail_bin) as i64;
    let mail: Option<String> = KV
      .zrangebyscore(K::MAIL_ID, mail_id, mail_id, false, Some((0, 1)))
      .await?;
    if let Some(mail) = mail {
      return Ok(reverse_mail(&mail));
    }
  }
  Ok("".to_owned())
}
