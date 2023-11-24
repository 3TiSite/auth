use r::{
  fred::interfaces::{RedisResult, SortedSetsInterface},
  KV,
};
use intbin::u64_bin;

use crate::K;

pub async fn host_mail_uid_bin(
  host_bin: impl AsRef<[u8]>,
  mail_id: u64,
) -> RedisResult<Option<Vec<u8>>> {
  let mail_bin = u64_bin(mail_id);
  KV.zscore(K::host_mail_uid(host_bin.as_ref()), mail_bin)
    .await
}
