use client::Client;
use intbin::bin_u64;
use t3::HeaderMap;
use xstr::lowtrim;

use crate::{
  db::{host_bin_mail_id, host_mail_uid_bin, uid_mail}, throw,
};

pub async fn post(header: HeaderMap, client: Client, json: String) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (uid, mail): (u64, String) = sonic_rs::from_str(&json)?;
  client.uid_logined(uid).await?;

  let mail = lowtrim(mail);
  let host = t3::origin_tld(&header)?;

  let (host_bin, mail_id) = host_bin_mail_id(&host, &mail).await?;

  if let Some(mail_id) = mail_id {
    if let Some(mail_uid) = host_mail_uid_bin(&host_bin, mail_id).await? {
      let mail_uid = bin_u64(mail_uid);
      if mail_uid != uid {
        throw!(header, mail, MAIL_USED)
      }
    }
  }
  let old_mail = uid_mail::get(host_bin, uid).await?;

  if !old_mail.is_empty() {
    // 找到老邮箱，给用户的老邮箱发送验证邮件
    dbg!("找到老邮箱，给用户的老邮箱发送验证邮件", old_mail);
  }
  // 给用户的新邮箱发送邮件

  // Ok(db::mail::host_send(i18n::RESET_PASSWORD, &header, host, account, password).await)

  Ok(())
}
