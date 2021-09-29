use crate::structures::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let queue = Queue::get(&ctx).await;

	let description = match queue.entry(interaction.guild_id.unwrap()) {
		Some(entry) => entry
			.requests
			.iter()
			.enumerate()
			.map(|(i, e)| format!("`{}.` {}\n", i + 1, e))
			.collect::<String>(),
		None => "The queue is empty!".into(),
	};

	interaction
		.embed(&ctx.http, |embed| embed.title("Queue").description(description))
		.await
}
