use anyhow::Result;
use m::mysql_async::Conn;
use r::fred::interfaces::HashesInterface;

use crate::{api::Token, db, K};

pub async fn new_with_conn(conn: &mut Conn, uid: u64, name: impl Into<String>) -> Result<Token> {
  let name = name.into();
  let (sk, day, sk_bin) = db::sk::day(uid);
  m::exe!(
      &mut (*conn);
      "INSERT INTO token(uid,sk,day,name) VALUES(?,?,?,?)",
      uid,
      sk,
      day,
      &name
  );

  let id = conn.last_insert_id().unwrap();
  let id_bin = &intbin::u64_bin(id)[..];
  r::KV.hset(K::TOKEN, (id_bin, &sk_bin[..])).await?;
  Ok(Token {
    id,
    sk: db::sk::b64(id, sk, day),
    name,
    enable: true,
  })
}

pub async fn new(uid: u64, name: impl Into<String>) -> Result<Token> {
  let mut conn = m::conn!();
  new_with_conn(&mut conn, uid, name).await
}
