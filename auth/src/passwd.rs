use client::Client;
use intbin::u64_bin;
use r::{fred::interfaces::HashesInterface, KV};
use t3::{ConnectInfo, HeaderMap};

use crate::{
  api, db,
  db::{code, host, passwd},
  i18n, throw, K,
};

pub async fn post(
  ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
  header: HeaderMap,
  client: Client,
  json: String,
) -> t3::msg!() {
  let (fingerprint, account, passwd, code): (String, String, String, String) =
    sonic_rs::from_str(&json)?;
  let account = xmail::norm(account);
  if !code::verify(i18n::RESET_PASSWORD, &account, &passwd, code) {
    throw!(header, code, CODE, INVALID)
  }
  let host_id = host::id_by_header(&header).await?;

  if let Some(uid) = m::authHostIdMailUid!(host_id, account) {
    trt::spawn(passwd::set(uid, passwd));
    let uid_bin = &u64_bin(uid)[..];
    let p = KV.pipeline();
    p.hget(K::NAME, uid_bin).await?;
    p.hget(K::LANG, uid_bin).await?;
    client
      .sign_in(&p, uid_bin, &header, &addr, fingerprint)
      .await?;
    let (name, lang, ..): (String, _, ()) = p.all().await?;

    return Ok(api::User {
      id: uid,
      name,
      lang: db::lang::get(lang) as _,
    });
  }
  throw!(header, account, ACCOUNT_NOT_EXIST)
}
