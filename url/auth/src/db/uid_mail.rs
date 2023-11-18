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
  Ok(id_mail(host_bin, uid).await?.1)
}

pub async fn id_mail(host_bin: impl AsRef<[u8]>, uid: u64) -> RedisResult<(u64, String)> {
  let mail_id;
  if let Some(mail_bin) = uid_mail_id::bin(host_bin, uid).await? {
    mail_id = bin_u64(mail_bin);
    let mail_id_i64 = mail_id as i64;
    let mail: Option<String> = KV
      .zrangebyscore(K::MAIL_ID, mail_id_i64, mail_id_i64, false, Some((0, 1)))
      .await?;
    if let Some(mail) = mail {
      return Ok((mail_id, reverse_mail(&mail)));
    }
  } else {
    mail_id = 0;
  }
  Ok((mail_id, "".to_owned()))
}
