use intbin::u64_bin;
use r::KV;
use tokio::join;

use crate::db;

pub async fn host_bin_mail_id(
  host: &str,
  account: impl AsRef<str>,
) -> t3::Result<(Box<[u8]>, Option<u64>)> {
  let account = account.as_ref();

  let (mail_id, host_id) = join!(
    db::id::mail(account),
    db::id::host::<Option<u64>, _>(&**KV, host)
  );

  let mail_id = mail_id?;
  let host_id = host_id?;

  let host_bin = u64_bin(tp::host_is_bind(host_id)?);

  Ok((host_bin, mail_id))
}
