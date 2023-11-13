#[macro_export]
macro_rules! client {
  ($client:ident, $json:ident, $action:ident) => {{
    use conn::{fred::interfaces::HashesInterface, KV};
    use intbin::u64_bin;
    use t3::ok;

    use crate::{api, K};
    let uid: u64 = sonic_rs::from_str(&$json)?;

    let client = $client;
    let p = KV.pipeline();
    client.$action(&p, &u64_bin(uid)).await?;
    p.all().await?;
    if let Some(uid) = client.uid().await? {
      return ok!(api::User {
        id: uid,
        name: KV.hget(K::NAME, u64_bin(uid)).await?
      });
    };

    return ok!(());
  }};
}
