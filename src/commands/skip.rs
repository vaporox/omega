use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result {
	let call = crate::get_call!(ctx, interaction);

	let handle = match call.lock().await.queue().current() {
		Some(handle) => handle,
		None => return interaction.reply(&ctx, replies::EMPTY_QUEUE).await,
	};

	handle.stop().unwrap();

	interaction
		.embed(&ctx, replies::track_embed(&handle, "Skipped Track"))
		.await
}
