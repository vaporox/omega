use crate::{commands, helpers::*};
use serde_json::Value;
use serenity::{
	async_trait,
	client::{Context, EventHandler},
	model::{gateway::Ready, id::GuildId, interactions::Interaction, voice::VoiceState},
};
use std::env;

pub struct Handler;

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
		if let Ok(Ok(guild_id)) = env::var("GUILD_ID").map(|var| var.parse::<u64>()) {
			let data = toml::from_str::<Value>(include_str!("../../commands.toml")).unwrap();
			let commands = data.pointer("/commands").unwrap();

			ctx.http
				.create_guild_application_commands(guild_id, commands)
				.await
				.or_print("set commands");
		}

		println!("{} is listening to {} guilds!", ready.user.name, ready.guilds.len());
	}

	async fn voice_state_update(&self, ctx: Context, _: Option<GuildId>, _: Option<VoiceState>, state: VoiceState) {
		if state.user_id != ctx.cache.current_user_id().await || state.channel_id.is_some() {
			return;
		}

		let manager = songbird::get(&ctx).await.unwrap();
		manager.remove(state.guild_id.unwrap()).await.ok();
	}
}
