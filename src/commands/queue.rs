use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let manager = songbird::get(&ctx).await.unwrap();

	let call = match manager.get(interaction.guild_id.unwrap()) {
		Some(call) => call,
		None => return interaction.reply(&ctx.http, "I'm not in a voice channel!").await,
	};

	let description = {
		let call = call.lock().await;
		let queue = call.queue().current_queue();

		match queue.len() {
			0 => "The queue is empty!".into(),
			_ => queue
				.iter()
				.enumerate()
				.map(|(i, e)| format!("`{}.` {}\n", i + 1, e.metadata().title.as_ref().unwrap()))
				.collect::<String>(),
		}
	};

	interaction
		.embed(&ctx.http, |embed| embed.title("Queue").description(description))
		.await
}
