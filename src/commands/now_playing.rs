use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<Message> {
	let call = crate::get_call!(ctx, interaction);

	let handle = match call.lock().await.queue().current() {
		Some(handle) => handle,
		None => return interaction.reply(&ctx, replies::EMPTY_QUEUE).await,
	};

	interaction
		.embed(&ctx, replies::track_embed_position(&handle, "Now Playing").await)
		.await
}
