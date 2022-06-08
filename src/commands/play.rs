use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<Message> {
	let guild_id = interaction.guild_id.unwrap();

	let voice_channel_id = match interaction.member.as_ref().unwrap().voice_channel_id(&ctx.cache) {
		Some(channel_id) => channel_id,
		None => return interaction.reply(&ctx, replies::USER_NOT_CONNECTED).await,
	};

	let option = interaction.data.options.get(0).unwrap();
	let track = option.value.as_ref().unwrap().as_str().unwrap();

	let input = match input::ytdl(track).await {
		Ok(input) => input,
		Err(_) => match input::ytdl_search(track).await {
			Ok(input) => input,
			Err(_) => return interaction.reply(&ctx, replies::NO_VIDEO).await,
		},
	};

	let manager = songbird::get(&ctx).await.unwrap();

	if let Some(call) = manager.get(guild_id) {
		let mut call = call.lock().await;
		let handle = call.enqueue_source(input);

		interaction
			.embed(&ctx, replies::track_embed(&handle, "Added to the Queue"))
			.await
	} else {
		let (arc, result) = manager.join(guild_id, voice_channel_id).await;
		result.unwrap();

		let mut call = arc.lock().await;

		call.deafen(true).await.unwrap();

		call.add_global_event(
			Event::Track(TrackEvent::End),
			VoiceHandler {
				cache: ctx.cache.clone(),
				call: arc.clone(),
				channel_id: interaction.channel_id,
				guild_id,
				http: ctx.http.clone(),
			},
		);

		let handle = call.enqueue_source(input);

		interaction
			.embed(&ctx, replies::track_embed(&handle, "Now Playing"))
			.await
	}
}
