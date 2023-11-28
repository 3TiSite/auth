use client::Client;

use crate::api;

// header: t3::HeaderMap,
pub async fn post(client: Client, json: String) -> t3::msg!() {
  let uid: u64 = sonic_rs::from_str(&json)?;

  client.uid_logined(uid).await?;

  // let host_id = host::id_by_header(&header)?;

  let mail = m::authUidMail!(uid);

  Ok(api::ConfMeta { mail })
}
