use crate::{helpers::InteractionHelpers, util::replies};
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let call = crate::get_call!(ctx, interaction);

	let description = {
		let call = call.lock().await;
		let queue = call.queue().current_queue();

		replies::current_queue(&queue)
	};

	if description.is_empty() {
		interaction.reply(&ctx.http, replies::EMPTY_QUEUE).await
	} else {
		interaction
			.embed(&ctx.http, |embed| embed.title("Queue").description(description))
			.await
	}
}
