use client::Client;
use jarg::{jarg, json};
use r::fred::interfaces::HashesInterface;

use crate::{db, K};

pub async fn post(client: Client, jarg!(id, enable): json!(u64, i8)) -> t3::msg!() {
  let uid = client.logined().await?;
  let enable = if enable == 0 { 0 } else { 1 };
  let li: Option<(u64, u64)> = m::q01!(format!("CALL tokenTurn({uid},{id},{enable})"));
  if let Some((sk, day)) = li {
    let id_bin = &intbin::u64_bin(id)[..];
    if day == 0 && sk == 0 {
      r::KV.hdel(K::TOKEN, id_bin).await?;
    } else {
      r::KV
        .hset(K::TOKEN, (id_bin, &db::sk::bin(sk, day, uid)[..]))
        .await?;
    }
  }
  Ok(())
}
