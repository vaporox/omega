use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let guild_id = interaction.guild_id.unwrap();
	let user_id = ctx.cache.current_user_id().await;

	let connected = ctx
		.cache
		.guild_field(guild_id, |guild| guild.voice_states.contains_key(&user_id))
		.await
		.unwrap();

	let content = if connected {
		let manager = songbird::get(&ctx).await.unwrap();
		manager.remove(guild_id).await.unwrap();

		replies::LEFT_CHANNEL
	} else {
		replies::BOT_NOT_CONNECTED
	};

	interaction.reply(&ctx.http, content).await
}
