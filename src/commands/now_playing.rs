use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let manager = songbird::get(&ctx).await.unwrap();

	let call = match manager.get(interaction.guild_id.unwrap()) {
		Some(call) => call,
		None => return interaction.reply(&ctx.http, "I'm not in a voice channel!").await,
	};

	let content = {
		let call = call.lock().await;

		match call.queue().current() {
			Some(handle) => format!("Now playing: **{}**", handle.metadata().title.as_ref().unwrap()),
			None => "There is nothing playing!".into(),
		}
	};

	interaction.reply(&ctx.http, content).await
}
