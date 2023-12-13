#[macro_export]
macro_rules! client {
  ($client:ident, $json:ident, $action:ident, $uid:ident) => {{
    use intbin::u64_bin;
    use r::KV;
    use t3::ok;
    let uid = $uid;
    let uid_bin = &u64_bin(uid)[..];
    let client = $client;
    let p = KV.pipeline();
    client.$action(&p, uid_bin).await?;
    client.zumax(&p).await?;
    let uid_bin: Option<Vec<u8>> = p.last().await?;
    client.set_uid_bin(uid_bin.clone());

    if let Some(uid_bin) = uid_bin {
      let user: $crate::api::User = $crate::db::api_user::by_id_bin(uid_bin).await?;
      return ok!(user);
    };

    return ok!(());
  }};
}
