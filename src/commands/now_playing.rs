use crate::{helpers::*, structures::*};
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let queue = Queue::get(&ctx).await;

	let content = match queue.entry(interaction.guild_id.unwrap()) {
		Some(entry) => format!("Now playing: **{}**", entry.requests[0]),
		None => "There is nothing playing!".into(),
	};

	interaction.reply(&ctx.http, content).await
}
