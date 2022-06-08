mod commands;
mod extensions;
mod handlers;
mod util;

use std::env;
use std::error::Error;

use handlers::Handler;
use serenity::client::Client;
use serenity::model::gateway::GatewayIntents;
use songbird::SerenityInit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let token = env::var("TOKEN")?;

	let mut client = Client::builder(token, GatewayIntents::non_privileged())
		.event_handler(Handler)
		.register_songbird()
		.await?;

	client.start().await?;

	Ok(())
}
