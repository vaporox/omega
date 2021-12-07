use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let call = crate::get_call!(ctx, interaction);

	let content = {
		let call = call.lock().await;

		match call.queue().current() {
			Some(handle) => replies::now_playing(handle.metadata().title.as_ref().unwrap()),
			None => replies::EMPTY_QUEUE.into(),
		}
	};

	interaction.reply(&ctx.http, content).await
}
