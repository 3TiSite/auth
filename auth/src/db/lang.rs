use r::fred::interfaces::{HashesInterface, RedisResult};

use crate::K;

pub async fn set<C: HashesInterface + Sync>(
  p: &C,
  uid_bin: &[u8],
  lang: impl AsRef<[u8]> + Send,
) -> RedisResult<()> {
  let lang = lang.as_ref();
  if lang.is_empty() {
    p.hdel(K::LANG, uid_bin).await?;
  } else {
    p.hset(K::LANG, (uid_bin, lang)).await?;
  }
  Ok(())
}

pub fn get(lang: Option<Vec<u8>>) -> u8 {
  if let Some(lang) = lang {
    return if lang.is_empty() { 0 } else { lang[0] };
  }
  0
}
