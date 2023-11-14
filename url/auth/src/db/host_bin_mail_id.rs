use conn::KV;
use intbin::u64_bin;

use crate::db;

pub async fn host_bin_mail_id(
  host: &str,
  account: impl AsRef<str>,
) -> t3::Result<(Box<[u8]>, Option<u64>)> {
  let p = KV.pipeline();
  db::id::host(&p, host).await?;
  db::id::mail(&p, account.as_ref()).await?;
  let (host_id, mail_id): (Option<u64>, Option<u64>) = p.all().await?;
  let host_bin = u64_bin(tp::host_is_bind(host_id)?);
  Ok((host_bin, mail_id))
}
