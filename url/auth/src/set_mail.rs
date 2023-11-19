use client::Client;
use conn::{fred::prelude::FunctionInterface, KV};
use intbin::u64_bin;
use t3::{ok, HeaderMap};

use crate::{db, db::code, i18n, lua, new_mail::host_old_mail_new_mail, throw, K};

pub async fn post(header: HeaderMap, client: Client, json: String) -> t3::msg!() {
  let (uid, new_mail, old_code, new_code): (u64, String, Option<String>, String) =
    sonic_rs::from_str(&json)?;

  let (host_bin, _, old_mail_id, old_mail, new_mail_id, new_mail) =
    host_old_mail_new_mail(&client, &header, uid, new_mail).await?;

  if !old_mail.is_empty() {
    #[allow(clippy::never_loop)]
    loop {
      if let Some(old_code) = old_code {
        if code::verify(
          i18n::MODIFY_MAIL,
          &old_mail,
          &new_mail,
          old_code.trim().to_owned(),
        ) {
          break;
        }
      }
      throw!(header, now, CODE, INVALID)
    }
  }

  if !code::verify(
    i18n::MODIFY_MAIL,
    &new_mail,
    &old_mail,
    new_code.trim().to_owned(),
  ) {
    throw!(header, mail, CODE, INVALID)
  }

  let new_mail_id = if let Some(id) = new_mail_id {
    id
  } else {
    db::id::mail_new_if_not_exist(&**KV, &new_mail).await?
  };

  let new_mail_id_bin = &u64_bin(new_mail_id);
  let uid_bin = u64_bin(uid);

  macro_rules! set_mail {
    ($($key:ident)? : $($arg:ident)?) => {
      KV.fcall(
        lua::UID_SET_MAIL,
        &[
          Box::from(K::UID_ACCOUNT),
          K::host_mail_uid(&host_bin),
          K::mail_uid(new_mail_id_bin),
          $($key)?
        ],
        [
          uid_bin,
          new_mail.as_bytes().into(),
          new_mail_id_bin.clone(),
          $($arg)?
        ]
      ).await?;
    };
  }
  if old_mail_id != 0 {
    let old_mail_id_bin = u64_bin(old_mail_id);
    let key = K::mail_uid(&old_mail_id_bin);
    set_mail!(key : old_mail_id_bin);
  } else {
    set_mail!(:);
  }
  ok!(())
}
