use anyhow::Result;

pub async fn set(host_id: u64, uid: u64, passwd: impl AsRef<str>) -> Result<()> {
  let passwd = passwd.as_ref().as_bytes();
  let salt = vb::e([host_id, uid]);
  let hash = passwd::hash(&salt, passwd);

  m::exe!(
    "INSERT INTO authPasswd (hostId,uid,hash) VALUES (?,?,?) ON DUPLICATE KEY UPDATE hash=VALUES(hash)",
    host_id,
    uid,
    hash
  );
  Ok(())
}

pub async fn exist(host_id: u64, uid: u64) -> Result<Option<Box<[u8]>>> {
  let r: Option<Vec<u8>> = m::q01!(
    "SELECT hash FROM authPasswd WHERE hostId=? AND uid=?",
    host_id,
    uid,
  );
  Ok(r.map(|i| i.into()))
}

pub fn verify_with_hash(host_id: u64, uid: u64, passwd: impl AsRef<str>, hash: &[u8]) -> bool {
  let passwd = passwd.as_ref().as_bytes();
  let salt = vb::e([host_id, uid]);
  return passwd::verify(&salt, passwd, hash);
}

pub async fn verify(host_id: u64, uid: u64, passwd: impl AsRef<str>) -> Result<bool> {
  let pre = exist(host_id, uid).await?;
  if let Some(pre) = pre {
    return Ok(verify_with_hash(host_id, uid, passwd, &pre));
  }
  Ok(false)
}
