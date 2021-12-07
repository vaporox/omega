use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let call = crate::get_call!(ctx, interaction);

	let cleared = {
		let call = call.lock().await;
		let queue = call.queue();
		let len = queue.len();

		queue.stop();
		len
	};

	let content = match cleared {
		0 => replies::EMPTY_QUEUE.into(),
		1 => replies::REMOVED_TITLE.into(),
		_ => replies::cleared_queue(cleared),
	};

	interaction.reply(&ctx.http, content).await
}
