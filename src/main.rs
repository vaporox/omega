mod commands;
mod handlers;
mod helpers;
mod util;

use handlers::Handler;
use serde_json::Value;
use serenity::client::Client;
use songbird::SerenityInit;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let application_id = env::var("APPLICATION_ID")?.parse::<u64>()?;
	let token = env::var("TOKEN")?;

	let mut client = Client::builder(token)
		.application_id(application_id)
		.event_handler(Handler)
		.register_songbird()
		.await?;

	let guild_ids = env::var("GUILD_IDS")?;
	let data = toml::from_str::<Value>(include_str!("../commands.toml"))?;
	let commands = data.pointer("/commands").ok_or("toml file did not contain commands")?;

	for guild_id in guild_ids.split(',') {
		if let Ok(guild_id) = guild_id.parse() {
			client
				.cache_and_http
				.http
				.create_guild_application_commands(guild_id, commands)
				.await?;
		};
	}

	client.start().await?;

	Ok(())
}
