mod commands;
mod helpers;
mod structures;

use helpers::*;
use serde_json::Value;
use serenity::{
	async_trait,
	model::{
		gateway::{Activity, Ready},
		id::GuildId,
		interactions::Interaction,
		prelude::VoiceState,
	},
	prelude::*,
};
use songbird::SerenityInit;
use std::{env, sync::Arc};
use structures::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let interaction = match interaction {
			Interaction::ApplicationCommand(command) => command,
			_ => return,
		};

		let result = if interaction.guild_id.is_none() {
			interaction.ephemeral(&ctx.http, "Commands are disabled in DMs!").await
		} else {
			match interaction.data.name.as_str() {
				"clear" => commands::clear::run(ctx, interaction).await,
				"leave" => commands::leave::run(ctx, interaction).await,
				"now-playing" => commands::now_playing::run(ctx, interaction).await,
				"play" => commands::play::run(ctx, interaction).await,
				"queue" => commands::queue::run(ctx, interaction).await,
				"remove" => commands::remove::run(ctx, interaction).await,
				"skip" => commands::skip::run(ctx, interaction).await,
				_ => interaction.ephemeral(&ctx.http, "Not implemented yet!").await,
			}
		};

		result.or_print("reply to command");
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		ctx.set_activity(Activity::listening("Mark Forster")).await;

		if let Ok(Ok(guild_id)) = env::var("GUILD_ID").map(|var| var.parse::<u64>()) {
			let data = toml::from_str::<Value>(include_str!("../commands.toml")).unwrap();
			let commands = data.pointer("/commands").unwrap();

			ctx.http
				.create_guild_application_commands(guild_id, commands)
				.await
				.or_print("set commands");
		}

		println!("{} is ready!\nGuilds: {}", ready.user.name, ready.guilds.len());
	}

	async fn voice_state_update(&self, ctx: Context, _: Option<GuildId>, _: Option<VoiceState>, state: VoiceState) {
		if state.user_id != ctx.cache.current_user_id().await || state.channel_id.is_some() {
			return;
		}

		let queue = Queue::get(&ctx).await;
		queue.clear(state.guild_id.unwrap());
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

	client.start().await.or_print("start client");
}
