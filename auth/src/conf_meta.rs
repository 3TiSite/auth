use client::Client;
use r::KV;
use intbin::u64_bin;

use crate::{api, db, db::uid_mail};

pub async fn post(header: t3::HeaderMap, client: Client, json: String) -> t3::msg!() {
  let uid: u64 = sonic_rs::from_str(&json)?;

  client.uid_logined(uid).await?;

  let host = &t3::origin_tld(&header)?;

  let host_bin = u64_bin(tp::host_is_bind(db::id::host(&**KV, host).await?)?);

  let mail = uid_mail::get(host_bin, uid).await?;

  Ok(api::ConfMeta { mail })
}
