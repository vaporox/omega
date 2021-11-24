use crate::{helpers::InteractionHelpers, util::replies};
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let call = crate::get_call!(ctx, interaction);

	let option = interaction.data.options.get(0).unwrap();

	let position: usize = match option.value.as_ref().unwrap().as_u64().and_then(|e| e.try_into().ok()) {
		Some(position) if position >= 1 => position,
		_ => return interaction.reply(&ctx.http, replies::INVALID_POSITION).await,
	};

	let content = {
		let call = call.lock().await;

		match call.queue().dequeue(position - 1) {
			Some(removed) => replies::removed_song(removed.metadata().title.as_ref().unwrap()),
			None => replies::INVALID_POSITION.into(),
		}
	};

	interaction.reply(&ctx.http, content).await
}
