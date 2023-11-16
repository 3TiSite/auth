#[macro_export]
macro_rules! client {
  ($client:ident, $json:ident, $action:ident) => {{
    use conn::{fred::interfaces::HashesInterface, KV};
    use intbin::u64_bin;
    use t3::ok;
    use $crate::{api, K};
    let uid: u64 = sonic_rs::from_str(&$json)?;
    let uid_bin = &u64_bin(uid)[..];

    let client = $client;
    let p = KV.pipeline();
    client.$action(&p, uid_bin).await?;
    p.all().await?;

    if let Some(uid) = client.uid().await? {
      let uid_bin = &u64_bin(uid)[..];
      let p = KV.pipeline();
      p.hget(K::NAME, uid_bin).await?;
      p.hget(K::LANG, uid_bin).await?;

      let (name, lang) = p.all().await?;
      let lang = $crate::db::lang::get(lang);

      return ok!(api::User {
        id: uid,
        name,
        lang: lang as _
      });
    };

    return ok!(());
  }};
}
