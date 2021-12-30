use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let guild_id = interaction.guild_id.unwrap();

	let voice_channel_id = match interaction.member.as_ref().unwrap().voice_channel_id(&ctx.cache).await {
		Some(channel_id) => channel_id,
		_ => return interaction.reply(&ctx.http, replies::USER_NOT_CONNECTED).await,
	};

	let option = interaction.data.options.get(0).unwrap();
	let request = option.value.as_ref().unwrap().as_str().unwrap();

	let input = match songbird::input::ytdl_search(request).await {
		Ok(input) => input,
		_ => return interaction.reply(&ctx.http, replies::NO_VIDEO).await,
	};

	let manager = songbird::get(&ctx).await.unwrap();

	let content = if let Some(call) = manager.get(guild_id) {
		let content = replies::added_song(input.metadata.title.as_deref().unwrap());

		let mut call = call.lock().await;
		call.enqueue_source(input);

		content
	} else {
		let (arc, result) = manager.join(guild_id, voice_channel_id).await;
		result.unwrap();

		let content = replies::now_playing(input.metadata.title.as_ref().unwrap());

		let mut call = arc.lock().await;

		call.deafen(true).await.unwrap();
		call.enqueue_source(input);

		call.add_global_event(
			Event::Track(TrackEvent::End),
			VoiceHandler {
				call: arc.clone(),
				channel_id: interaction.channel_id,
				http: ctx.http.clone(),
			},
		);

		content
	};

	interaction.reply(&ctx.http, content).await
}
