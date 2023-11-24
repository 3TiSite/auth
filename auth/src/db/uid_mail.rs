use anyhow::Result;
use intbin::bin_u64;

use crate::db::uid_mail_id;

pub async fn get(host_bin: impl AsRef<[u8]>, uid: u64) -> Result<String> {
  Ok(id_mail(host_bin, uid).await?.1)
}

pub async fn id_mail(host_bin: impl AsRef<[u8]>, uid: u64) -> Result<(u64, String)> {
  let mail_id;
  if let Some(mail_bin) = uid_mail_id::bin(host_bin, uid).await? {
    mail_id = bin_u64(mail_bin);
    let mail: Option<String> = db::q01!("SELECT idMail(?)", mail_id);
    if let Some(mail) = mail {
      return Ok((mail_id, mail));
    }
  } else {
    mail_id = 0;
  }
  Ok((mail_id, "".to_owned()))
}
