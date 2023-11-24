use client::Client;
use r::KV;
use intbin::u64_bin;
use t3::HeaderMap;

use crate::db;

pub async fn post(header: HeaderMap, client: Client, _json: String) -> t3::msg!() {
  if let Some(uid) = client.uid().await? {
    let uid_bin = u64_bin(uid);
    let lang = lang::header_bin(&header);
    db::lang::set(&**KV, &uid_bin, lang).await?;
  }
  Ok(())
}
