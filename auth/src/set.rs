use client::Client;
use jarg::{jarg, json};

use crate::client;

pub async fn post(client: Client, jarg!(uid): json!(u64)) -> t3::msg!() {
  client!(client, json, set, uid)
}
