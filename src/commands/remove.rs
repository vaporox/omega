use crate::{helpers::*, structures::*};
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};
use std::convert::TryInto;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let option = interaction.data.options.get(0).unwrap();

	let position: usize = match option.value.as_ref().unwrap().as_u64().and_then(|e| e.try_into().ok()) {
		Some(position) if position >= 1 => position,
		_ => return interaction.reply(&ctx.http, "Invalid position!").await,
	};

	let queue = Queue::get(&ctx).await;

	let content = match queue.remove(interaction.guild_id.unwrap(), position - 1) {
		Some(removed) => format!("Removed from the queue: **{}**", removed),
		None => "Invalid position!".into(),
	};

	interaction.reply(&ctx.http, content).await
}
