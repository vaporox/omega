use crate::structures::*;
use serenity::{
	async_trait,
	client::Context,
	model::interactions::application_command::{
		ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
	},
	Result,
};
use std::convert::TryInto;

pub struct RemoveCommand;

#[async_trait]
impl Command for RemoveCommand {
	async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
		let option = interaction.data.options.get(0).unwrap();

		let position: usize = match option.resolved.as_ref().unwrap() {
			ApplicationCommandInteractionDataOptionValue::Integer(index) => match (*index).try_into() {
				Ok(position) if position >= 1 => position,
				_ => return interaction.reply(&ctx.http, "Invalid position!").await,
			},
			_ => unreachable!(),
		};

		let queue = Queue::get(&ctx).await;

		let content = match queue.remove(interaction.guild_id.unwrap(), position - 1) {
			Some(removed) => format!("Removed from the queue: **{}**", removed),
			None => "Invalid position!".into(),
		};

		interaction.reply(&ctx.http, content).await
	}
}
