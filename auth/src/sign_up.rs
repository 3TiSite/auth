use client::Client;
use intbin::u64_bin;
use r::{fred::interfaces::HashesInterface, KV};
use t3::{ConnectInfo, HeaderMap};
use xmail::norm_tld;

use crate::{
  api,
  db::{bantld, code, host::id_by_header, lang, name, passwd},
  i18n, throw,
  K::{self},
};

pub async fn post(
  ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
  header: HeaderMap,
  client: Client,
  json: String,
) -> t3::msg!() {
  let (fingerprint, account, passwd, verify_code, name): (String, String, String, String, String) =
    sonic_rs::from_str(&json)?;

  let mut name = name.trim().to_owned();
  if name.is_empty() {
    name = if let Some(p) = account.find('@') {
      account[..p].into()
    } else {
      account.clone()
    }
    .trim()
    .into();
  };

  let (account, tld) = norm_tld(account);
  if bantld::is(tld).await? {
    throw!(header, code, BAN_MAIL)
  }

  if !code::verify(i18n::SIGN_UP, &account, &passwd, verify_code) {
    throw!(header, code, CODE, INVALID)
  }
  let host_id = id_by_header(&header).await?;

  let uid: u64 = m::authUidMailNew!(host_id, &account);
  let uid_bin = &u64_bin(uid)[..];

  if let Some(hash) = passwd::exist(host_id, uid).await? {
    if !passwd::verify_with_hash(host_id, uid, passwd, &hash) {
      throw!(header, code, ACCOUNT_EXIST)
    }
  } else {
    trt::spawn!(passwd::set(host_id, uid, passwd));
  }

  let name = name::truncate(name);
  let name = name.as_bytes();
  let lang = ::lang::header_bin(&header);

  let p = KV.pipeline();
  p.hset(K::UID_ACCOUNT, (uid_bin, account)).await?;
  p.hset(K::NAME, (uid_bin, name)).await?;
  lang::set(&p, uid_bin, lang).await?;
  client
    .sign_in(&p, uid_bin, &header, &addr, fingerprint)
    .await?;
  p.all().await?;

  Ok(api::Uid { id: uid })
}
