mod commands;
mod handlers;
mod helpers;
mod util;

use handlers::Handler;
use serenity::client::Client;
use songbird::SerenityInit;
use std::env;

#[tokio::main]
async fn main() {
	dotenv::dotenv().unwrap();

	let application_id = env::var("APPLICATION_ID").unwrap().parse::<u64>().unwrap();
	let token = env::var("TOKEN").unwrap();

	let mut client = Client::builder(token)
		.application_id(application_id)
		.event_handler(Handler)
		.register_songbird()
		.await
		.unwrap();

	client.start().await.unwrap();
}
