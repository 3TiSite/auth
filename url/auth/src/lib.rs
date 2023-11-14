t3::api!();

urlmod!();
mod _mod;
mod db;
mod r#macro;
use crate::db::sign_in::{sign_in, SignIn};
#[allow(non_snake_case)]
pub mod K;
mod i18n;
mod lua;

use anyhow::Result;
use axum::http::header::HeaderMap;
use client::Client;
use conn::{fred::interfaces::HashesInterface, KV};
use intbin::u64_bin;
use t3::ok;
use xstr::lowtrim;

pub const SIGN_UP: u8 = 0; // 注册
pub const SIGN_IN: u8 = 1; // 登录

pub async fn sign_in_name(client: &Client, id: u64) -> Result<String> {
  let id_bin = &u64_bin(id)[..];
  let client_uid = client.uid().await?;
  let set = if client_uid.is_none() {
    true
  } else {
    id != client_uid.unwrap()
  };

  Ok(if set {
    let p = KV.pipeline();
    p.hget(K::NAME, id_bin).await?;
    client.sign_in(&p, id_bin).await?;
    let li: (String, (), ()) = p.all().await?;
    li.0
  } else {
    KV.hget(K::NAME, id_bin).await?
  })
}

pub async fn post(client: Client, header: HeaderMap, json: String) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (action, account, password): (u8, String, String) = sonic_rs::from_str(&json)?;

  let account = lowtrim(account);
  let host = &t3::origin_tld(&header)?;

  match sign_in(host, &account, &password).await? {
    SignIn::Ok(id) => {
      let name = sign_in_name(&client, id).await?;
      ok!(api::User { id, name })
    }
    SignIn::PasswdError => {
      if action == SIGN_UP {
        throw!(header, account, ACCOUNT_EXIST);
      }
      throw!(header, password, PASSWORD_ERROR);
    }
    SignIn::AccountNotExist => {
      if action == SIGN_UP {
        return ok!(db::mail::host_send(i18n::SIGN_UP, &header, host, account, password).await?);
      }
      throw!(header, account, ACCOUNT_NOT_EXIST);
    }
  }
}
