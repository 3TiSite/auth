use client::Client;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  let uid: u64 = sonic_rs::from_str(&json)?;
  client.exit(uid).await?;
  return Ok(());
}
