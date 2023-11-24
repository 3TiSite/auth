use client::Client;

pub async fn post(client: Client) -> t3::msg!() {
  Ok(client.exit_all().await?)
}
