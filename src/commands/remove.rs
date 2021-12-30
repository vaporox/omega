use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let call = crate::get_call!(ctx, interaction);

	let option = interaction.data.options.get(0).unwrap();

	let position: usize = match option.value.as_ref().unwrap().as_u64().and_then(|e| e.try_into().ok()) {
		Some(position) if position >= 1 => position,
		_ => return interaction.reply(&ctx.http, replies::INVALID_POSITION).await,
	};

	let content = {
		let call = call.lock().await;

		let handle = if position == 1 {
			call.queue().current()
		} else {
			call.queue().dequeue(position - 1).map(|e| e.handle())
		};

		match handle {
			Some(removed) => {
				if position == 1 {
					removed.stop().unwrap();
				}
				replies::removed_song(removed.metadata().title.as_ref().unwrap())
			}
			None => replies::INVALID_POSITION.into(),
		}
	};

	interaction.reply(&ctx.http, content).await
}
