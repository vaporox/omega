use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result {
	let call = crate::get_call!(ctx, interaction);

	let cleared = {
		let call = call.lock().await;
		let queue = call.queue();
		let len = queue.len();

		queue.stop();
		len
	};

	interaction.reply(&ctx, replies::cleared_queue(cleared)).await
}
