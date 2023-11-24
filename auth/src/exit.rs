use client::Client;

use crate::client;

pub async fn post(client: Client, json: String) -> t3::msg!() {
  client!(client, json, exit)
}
