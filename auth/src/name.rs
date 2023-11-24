use client::Client;
use r::{fred::interfaces::HashesInterface, KV};
use intbin::u64_bin;

use crate::{db::name, K};

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let (uid, name): (u64, String) = sonic_rs::from_str(&json)?;
  let name = name::truncate(name);
  client.uid_logined(uid).await?;
  KV.hset(K::NAME, (u64_bin(uid), name.as_bytes())).await?;
  Ok(())
}
