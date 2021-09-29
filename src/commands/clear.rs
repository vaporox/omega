use crate::structures::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let queue = Queue::get(&ctx).await;

	let content = match queue.clear(interaction.guild_id.unwrap()) {
		0 => "There is nothing playing!".into(),
		1 => "Removed **1** title from the queue!".into(),
		removed => format!("Removed **{}** titles from the queue!", removed),
	};

	interaction.reply(&ctx.http, content).await
}
