use crate::structures::*;
use serenity::{
	async_trait, client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result,
};

pub struct ClearCommand;

#[async_trait]
impl Command for ClearCommand {
	async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
		let queue = Queue::get(&ctx).await;

		let content = match queue.clear(interaction.guild_id.unwrap()) {
			0 => "There is nothing playing!".into(),
			1 => "Removed **1** title from the queue!".into(),
			removed => format!("Removed **{}** titles from the queue!", removed),
		};

		interaction.reply(&ctx.http, content).await
	}
}
