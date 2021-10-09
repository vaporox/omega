use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let manager = songbird::get(&ctx).await.unwrap();

	let call = match manager.get(interaction.guild_id.unwrap()) {
		Some(call) => call,
		None => return interaction.reply(&ctx.http, "I'm not in a voice channel!").await,
	};

	let cleared = {
		let call = call.lock().await;
		let queue = call.queue();
		let len = queue.len();

		queue.stop();
		len
	};

	let content = match cleared {
		0 => "There is nothing playing!".into(),
		1 => "Removed **1** title from the queue!".into(),
		removed => format!("Removed **{}** titles from the queue!", removed),
	};

	interaction.reply(&ctx.http, content).await
}
