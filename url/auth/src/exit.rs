use client::Client;
use conn::KV;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let uid: u64 = sonic_rs::from_str(&json)?;
  let p = KV.pipeline();
  client.exit(&p, &intbin::u64_bin(uid)).await?;
  p.all().await?;
  return Ok(());
}
