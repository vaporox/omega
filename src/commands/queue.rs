use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<Message> {
	let call = crate::get_call!(ctx, interaction);

	let queue = call.lock().await.queue().current_queue();

	if queue.is_empty() {
		return interaction.reply(&ctx, replies::EMPTY_QUEUE).await;
	}

	interaction.embed(&ctx, replies::queue_embed(&queue)).await
}
