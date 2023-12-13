use jarg::{jarg, json};
use t3::HeaderMap;

use crate::{
  db::{host, mail},
  i18n, throw,
};

pub async fn post(
  _: captcha::Captcha,
  header: HeaderMap,
  jarg!(account, password): json!(String, String),
) -> t3::msg!() {
  let account = xmail::norm(account);
  let (host, host_id) = host::by_header(&header).await?;

  if m::authHostIdMailUid!(host_id, &account).is_some() {
    return mail::host_send(i18n::RESET_PASSWORD, &header, &host, account, password).await;
  }

  throw!(header, account, ACCOUNT_NOT_EXIST)
}
