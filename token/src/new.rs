use client::Client;
use jarg::{jarg, json};

use crate::{api, db};

pub async fn post(client: Client, jarg!(name): json!(String)) -> t3::msg!() {
  let uid = client.logined().await?;
  let s = name.trim();
  let name = &s[..s.len().min(32)];

  Ok::<api::Token, _>(db::new(uid, name).await?)
}
