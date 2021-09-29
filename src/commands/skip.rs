use crate::structures::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let guild_id = interaction.guild_id.unwrap();
	let queue = Queue::get(&ctx).await;

	let content = match queue.skip(guild_id) {
		Some(skipped) => format!("Skipped: **{}**", skipped),
		None => "There is nothing playing!".into(),
	};

	let manager = songbird::get(&ctx).await.unwrap();

	if let Some(call) = manager.get(guild_id) {
		let mut call = call.lock().await;

		if let Some(entry) = queue.entry(guild_id) {
			VoiceHandler::play(&mut call, &entry.requests[0]).await;
		} else {
			call.stop();
		}
	}

	interaction.reply(&ctx.http, content).await
}
