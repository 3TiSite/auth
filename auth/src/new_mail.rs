use client::Client;
use jarg::{jarg, json};
use t3::{ok, HeaderMap};
use xmail::norm_tld;

use crate::{
  api,
  db::{bantld, host::by_header, mail},
  i18n, throw,
};

pub async fn host_old_mail_new_mail(
  client: &Client,
  header: &HeaderMap,
  uid: u64,
  new_mail: String,
) -> t3::Result<(String, String, String)> {
  client.uid_logined(uid).await?;
  let (new_mail, tld) = norm_tld(new_mail);
  if bantld::is(tld).await? {
    throw!(header, mail, BAN_MAIL)
  }

  let (host, host_id) = by_header(&header).await?;

  if m::authHostIdMailUid!(host_id, &new_mail).is_some() {
    throw!(header, mail, MAIL_USED)
  }
  let old_mail = m::authUidMail!(uid);
  Ok((host, old_mail, new_mail))
}

pub async fn post(
  _: captcha::Captcha,
  header: HeaderMap,
  client: Client,
  jarg!(uid, new_mail): json!(u64, String),
) -> t3::msg!() {
  let (host, old_mail, new_mail) = host_old_mail_new_mail(&client, &header, uid, new_mail).await?;

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
