use client::Client;
use conn::{fred::interfaces::HashesInterface, KV};
use t3::ok;

use crate::{api, K};

pub async fn post(client: Client) -> t3::msg!() {
  if let Some(id) = client.uid().await? {
    let id_bin = intbin::u64_bin(id);
    let name: String = KV.hget(K::NAME, id_bin).await?;
    return ok!(api::User { id, name });
  }
  ok!(())
}
