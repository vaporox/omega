use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let guild_id = interaction.guild_id.unwrap();
	let user_id = ctx.cache.current_user_id().await;

	let connected = ctx
		.cache
		.guild_field(guild_id, |guild| guild.voice_states.contains_key(&user_id))
		.await
		.unwrap();

	let content = if connected {
		let manager = songbird::get(&ctx).await.unwrap();
		manager.remove(guild_id).await.or_print("leave voice channel");

		"Left the voice channel!"
	} else {
		"I'm not in a voice channel!"
	};

	interaction.reply(&ctx.http, content).await
}
