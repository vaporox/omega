use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let call = crate::get_call!(ctx, interaction);

	let description = {
		let call = call.lock().await;
		let queue = call.queue().current_queue();

		queue
			.iter()
			.enumerate()
			.map(|(i, e)| format!("`{}.` {}\n", i + 1, e.metadata().title.as_ref().unwrap()))
			.collect::<String>()
	};

	if description.is_empty() {
		interaction.reply(&ctx.http, "There is nothing playing!").await
	} else {
		interaction
			.embed(&ctx.http, |embed| embed.title("Queue").description(description))
			.await
	}
}
