use client::Client;
use jarg::{jarg, json};
use r::fred::interfaces::HashesInterface;

use crate::K;

pub async fn post(client: Client, jarg!(token_id): json!(u64)) -> t3::msg!() {
  let uid = client.logined().await?;

  if m::tokenRm!(uid, token_id) > 0 {
    r::KV.hdel(K::TOKEN, intbin::u64_bin(token_id)).await?;
  }

  Ok(())
}
