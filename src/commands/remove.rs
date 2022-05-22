use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result {
	let call = crate::get_call!(ctx, interaction);

	if call.lock().await.queue().is_empty() {
		return interaction.reply(&ctx, replies::EMPTY_QUEUE).await;
	}

	let option = interaction.data.options.get(0).unwrap();

	let position: usize = match option.value.as_ref().unwrap().as_i64().unwrap().try_into() {
		Ok(position) => position,
		Err(_) => return interaction.reply(&ctx, replies::INVALID_POSITION).await,
	};

	let handle = {
		let call = call.lock().await;

		if position == 1 {
			let handle = call.queue().current();

			if let Some(handle) = &handle {
				handle.stop().unwrap();
			}

			handle
		} else {
			call.queue().dequeue(position - 1).map(|e| e.handle())
		}
	};

	let handle = match handle {
		Some(handle) => handle,
		None => return interaction.reply(&ctx, replies::INVALID_POSITION).await,
	};

	interaction
		.embed(&ctx, replies::track_embed(&handle, "Removed from the Queue"))
		.await
}
