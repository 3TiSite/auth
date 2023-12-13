use client::Client;
use jarg::{jarg, json};
use r::fred::interfaces::HashesInterface;
use t3::ok;

use crate::{api, db, K};

pub async fn post(client: Client, jarg!(token_id): json!(u64)) -> t3::msg!() {
  let uid = client.logined().await?;
  let (sk, day, sk_bin) = db::sk::day(uid);

  let id_bin = &intbin::u64_bin(token_id)[..];

  if m::tokenRefresh!(token_id, uid, sk, day) > 0 {
    r::KV.hset(K::TOKEN, (id_bin, &sk_bin[..])).await?;
    return ok!(api::Sk {
      v: db::sk::b64(token_id, sk, day)
    });
  }

  ok!(())
}
