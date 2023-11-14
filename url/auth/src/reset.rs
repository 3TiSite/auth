use conn::{fred::interfaces::SortedSetsInterface, KV};
use intbin::u64_bin;
use t3::HeaderMap;
use xstr::lowtrim;

use crate::{db, db::host_bin_mail_id, i18n, throw, K};

pub async fn post(header: HeaderMap, json: String) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (account, password): (String, String) = sonic_rs::from_str(&json)?;

  let account = lowtrim(account);
  let host = &t3::origin_tld(&header)?;
  let (host_id, mail_id) = host_bin_mail_id(host, &account).await?;

  if let Some(mail_id) = mail_id {
    let mail_bin = u64_bin(mail_id);
    let uid: Option<Vec<u8>> = KV.zscore(K::host_mail_uid(&host_id), mail_bin).await?;
    if uid.is_some() {
      return db::mail::host_send(i18n::RESET_PASSWORD, &header, host, account, password).await;
    }
  }

  throw!(header, account, ACCOUNT_NOT_EXIST)
}
