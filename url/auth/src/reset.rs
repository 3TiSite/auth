use t3::HeaderMap;

use crate::{
  db,
  db::{host_bin_mail_id, host_mail_uid_bin},
  i18n, throw,
};

pub async fn post(header: HeaderMap, json: String) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (account, password): (String, String) = sonic_rs::from_str(&json)?;

  let account = xmail::norm(account);
  let host = &t3::origin_tld(&header)?;
  let (host_bin, mail_id) = host_bin_mail_id(host, &account).await?;

  if let Some(mail_id) = mail_id {
    if host_mail_uid_bin(host_bin, mail_id).await?.is_some() {
      return db::mail::host_send(i18n::RESET_PASSWORD, &header, host, account, password).await;
    }
  }

  throw!(header, account, ACCOUNT_NOT_EXIST)
}
