mod commands;
mod structures;

use serde_json::Value;
use serenity::{
	async_trait,
	model::{
		gateway::{Activity, Ready},
		interactions::Interaction,
	},
	prelude::*,
};
use songbird::SerenityInit;
use std::{env, sync::Arc};
use structures::{Command, Queue, Responses};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let interaction = match interaction {
			Interaction::ApplicationCommand(command) => command,
			_ => unreachable!(),
		};

		let result = if interaction.guild_id.is_none() {
			interaction.ephemeral(&ctx.http, "Commands are disabled in DMs!")
		} else {
			match interaction.data.name.as_str() {
				"clear" => commands::ClearCommand::run(ctx, interaction),
				"leave" => commands::LeaveCommand::run(ctx, interaction),
				"now-playing" => commands::NowPlayingCommand::run(ctx, interaction),
				"play" => commands::PlayCommand::run(ctx, interaction),
				"queue" => commands::QueueCommand::run(ctx, interaction),
				"remove" => commands::RemoveCommand::run(ctx, interaction),
				"skip" => commands::SkipCommand::run(ctx, interaction),
				_ => interaction.ephemeral(&ctx.http, "Not implemented yet!"),
			}
		};

		if let Err(error) = result.await {
			eprintln!("Error replying to command: {}", error);
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		ctx.set_activity(Activity::listening("Mark Forster")).await;

		if let Ok(Ok(guild_id)) = env::var("GUILD_ID").map(|var| var.parse::<u64>()) {
			let data = toml::from_str::<Value>(include_str!("../commands.toml")).unwrap();
			let commands = data.pointer("/commands").unwrap();

			if let Err(error) = ctx.http.create_guild_application_commands(guild_id, commands).await {
				eprintln!("Error setting commands: {}", error);
			};
		}

		println!("{} is ready!\nGuilds: {}", ready.user.name, ready.guilds.len());
	}
}

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

	{
		let mut data = client.data.write().await;
		data.insert::<Queue>(Arc::new(Queue::default()));
	}

	if let Err(error) = client.start().await {
		eprintln!("Error starting client: {}", error);
	}
}
