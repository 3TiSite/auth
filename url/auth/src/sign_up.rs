use client::Client;
use conn::{
  fred::interfaces::{FunctionInterface, HashesInterface},
  KV,
};
use intbin::{bin_u64, u64_bin};
use t3::HeaderMap;
use tokio::task::spawn_blocking;
use xstr::lowtrim;

use crate::{
  api, db,
  db::code,
  i18n, lua, throw,
  K::{self, MAIL_ID},
};

pub async fn post(header: HeaderMap, client: Client, json: String) -> t3::msg!() {
  let (account, password, verify_code, name): (String, String, String, String) =
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

  let account = lowtrim(account);
  if !code::verify(i18n::SIGN_UP, &account, &password, verify_code) {
    throw!(header, code, CODE, INVALID);
  }
  let host = t3::origin_tld(&header)?;
  let p = KV.pipeline();
  db::id::host(&p, &host).await?;
  p.fcall(lua::ZSET_ID, &[MAIL_ID], [db::id::reverse_mail(&account)])
    .await?;

  let (host_id, mail_id): (Option<u64>, u64) = p.all().await?;

  let host_id = tp::host_is_bind(host_id)?;
  let mail_id_bin = &u64_bin(mail_id);
  let host_id_bin = &u64_bin(host_id)[..];

  let mut r: Vec<Vec<u8>> = KV
    .fcall(
      lua::ACCOUNT_NEW_UID_PASSWD,
      &[
        &K::host_mail_uid(host_id_bin),
        K::UID,
        &K::mail_uid(mail_id_bin),
        K::UID_ACCOUNT,
        K::UID_HOST,
        K::UID_PASSWD,
      ],
      [
        mail_id_bin,
        account.as_bytes(),
        host_id_bin,
        &passwd::hash(password.as_bytes())[..],
      ],
    )
    .await?;

  if r.len() == 2 {
    let hash = r.pop().unwrap();
    if !spawn_blocking(move || passwd::verify(password.as_bytes(), &hash)).await? {
      throw!(header, account, ACCOUNT_EXIST);
    }
  }

  let uid_bin = &r[0][..];
  let id = bin_u64(uid_bin);
  let name = name.as_bytes();
  let lang = lang::header_bin(&header);

  let p = KV.pipeline();
  p.hset(K::NAME, (uid_bin, name)).await?;
  db::lang::set(&p, uid_bin, lang).await?;
  client.sign_in(&p, uid_bin).await?;
  p.all().await?;

  Ok(api::Uid { id })
}
