use client::Client;

use crate::{api, db};

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let uid = client.logined().await?;
  let name: String = sonic_rs::from_str(&json)?;
  let name = name.trim();

  Ok::<api::Token, _>(db::new(uid, name).await?)
}
