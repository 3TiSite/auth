use client::Client;
use conn::{fred::interfaces::HashesInterface, KV};
use intbin::u64_bin;

use crate::K;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let (uid, name): (u64, String) = sonic_rs::from_str(&json)?;
  client.uid_logined(uid).await?;
  KV.hset(K::NAME, (u64_bin(uid), name)).await?;
  Ok(())
}
