use std::ops::Deref;

use client::Client;
use conn::{
  fred::interfaces::{HashesInterface, SortedSetsInterface},
  KV,
};
use intbin::{bin_u64, u64_bin};

use crate::{api, db, db::id::reverse_mail, K};

pub async fn post(header: t3::HeaderMap, client: Client, json: String) -> t3::msg!() {
  let uid: u64 = sonic_rs::from_str(&json)?;
  if client.is_login(uid).await? {
    let host = &t3::origin_tld(&header)?;
    let host_bin = u64_bin(db::id::host(&**KV, host).await?);

    let mut mail_id: Vec<Vec<u8>> = KV
      .zrangebyscore(
        K::host_mail_uid(&host_bin),
        uid as i64,
        uid as i64,
        false,
        Some((0, 1)),
      )
      .await?;
    if !mail_id.is_empty() {
      let mail_id = bin_u64(mail_id.pop().unwrap()) as i64;
      let mut mail: Vec<String> = KV
        .zrangebyscore(K::MAIL_ID, mail_id, mail_id, false, Some((0, 1)))
        .await?;
      if !mail.is_empty() {
        let mail = mail.pop().unwrap();
        return Ok(api::ConfMeta {
          mail: reverse_mail(&mail),
        });
      }
    }
  }
  t3::err(t3::StatusCode::UNAUTHORIZED, ())
}
