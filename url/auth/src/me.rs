use client::Client;
use conn::{fred::interfaces::HashesInterface, KV};
use t3::ok;

use crate::{api, db, K};

pub async fn post(client: Client) -> t3::msg!() {
  if let Some(id) = client.uid().await? {
    let id_bin = &intbin::u64_bin(id)[..];
    let p = KV.pipeline();
    p.hget(K::NAME, id_bin).await?;
    p.hget(K::LANG, id_bin).await?;
    let (name, lang): (String, _) = p.all().await?;
    let lang = db::lang::get(lang);
    return ok!(api::User {
      id,
      name,
      lang: lang as _
    });
  }
  ok!(())
}
