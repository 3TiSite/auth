use client::Client;
use r::fred::interfaces::HashesInterface;

use crate::K;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let token_id: u64 = sonic_rs::from_str(&json)?;
  let uid = client.logined().await?;

  if m::tokenRm!(uid, token_id) > 0 {
    r::KV.hdel(K::TOKEN, intbin::u64_bin(token_id)).await?;
  }

  Ok(())
}
