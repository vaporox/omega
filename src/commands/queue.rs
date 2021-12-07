use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
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
