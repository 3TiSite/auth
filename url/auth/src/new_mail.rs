use client::Client;
use intbin::bin_u64;
use t3::{ok, HeaderMap};
use xstr::lowtrim;

use crate::{
  api,
  db::{host_bin_mail_id, host_mail_uid_bin, mail, uid_mail},
  i18n, throw,
};

pub async fn host_old_mail_new_mail(
  client: &Client,
  header: &HeaderMap,
  uid: u64,
  new_mail: String,
) -> t3::Result<(Box<[u8]>, String, u64, String, Option<u64>, String)> {
  client.uid_logined(uid).await?;
  let new_mail = lowtrim(new_mail);
  let host = t3::origin_tld(header)?;
  let (host_bin, new_mail_id) = host_bin_mail_id(&host, &new_mail).await?;

  if let Some(new_mail_id) = new_mail_id {
    if let Some(mail_uid) = host_mail_uid_bin(&host_bin, new_mail_id).await? {
      let mail_uid = bin_u64(mail_uid);
      if mail_uid != uid {
        throw!(header, mail, MAIL_USED)
      }
    }
  }
  let (old_mail_id, old_mail) = uid_mail::id_mail(&host_bin, uid).await?;
  Ok((host_bin, host, old_mail_id, old_mail, new_mail_id, new_mail))
}

pub async fn post(header: HeaderMap, client: Client, json: String) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (uid, new_mail): (u64, String) = sonic_rs::from_str(&json)?;
  let (_, host, _, old_mail, _, new_mail) =
    host_old_mail_new_mail(&client, &header, uid, new_mail).await?;

  if !old_mail.is_empty() {
    let suffix = format!("{old_mail} → {new_mail}");
    mail::host_send_with_suffix(
      i18n::MODIFY_MAIL,
      &header,
      &host,
      &old_mail,
      &new_mail,
      &suffix,
    )
    .await?;
    mail::host_send_with_suffix(
      i18n::MODIFY_MAIL,
      &header,
      &host,
      &new_mail,
      &old_mail,
      &suffix,
    )
    .await?;
    return ok!(api::Mail { mail: old_mail });
  }

  mail::host_send(i18n::MODIFY_MAIL, &header, &host, &new_mail, &old_mail).await?;

  ok!(())
}
