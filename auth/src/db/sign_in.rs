use r::{fred::interfaces::FunctionInterface, KV};
use intbin::{bin_u64, u64_bin};
use tokio::task::spawn_blocking;

use crate::{db::host_bin_mail_id, lua, K};

pub enum SignIn {
  Ok(u64),
  AccountNotExist,
  PasswdError,
}

pub async fn sign_in(host: &str, account: &str, password: impl Into<String>) -> t3::Result<SignIn> {
  let (host_bin, mail_id) = host_bin_mail_id(host, account).await?;
  if let Some(mail_id) = mail_id {
    let mail_bin = u64_bin(mail_id);
    let uid_passwd: Option<Vec<Vec<u8>>> = KV
      .fcall_ro(
        lua::MAIL_UID_PASSWD,
        &[&K::host_mail_uid(&host_bin), K::UID_PASSWD],
        [mail_bin],
      )
      .await?;
    if let Some(uid_passwd) = uid_passwd {
      if uid_passwd.len() >= 2 {
        let uid_bin = &uid_passwd[0];
        let uid = bin_u64(uid_bin);
        let hash = uid_passwd[1].clone();
        let password: Box<_> = password.into().as_bytes().into();
        if spawn_blocking(move || passwd::verify(&password, &hash)).await? {
          return Ok(SignIn::Ok(uid));
        }
      }
      return Ok(SignIn::PasswdError);
    }
  }
  Ok(SignIn::AccountNotExist)
}
