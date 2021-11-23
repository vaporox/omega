use crate::{handlers::VoiceHandler, helpers::*};
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};
use songbird::{Event, TrackEvent};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let guild_id = interaction.guild_id.unwrap();

	let voice_channel_id = match interaction.member.as_ref().unwrap().voice_channel_id(&ctx).await {
		Some(channel_id) => channel_id,
		_ => return interaction.reply(&ctx.http, "You're not in a voice channel!").await,
	};

	let option = interaction.data.options.get(0).unwrap();
	let request = option.value.as_ref().unwrap().as_str().unwrap();

	let input = match songbird::ytdl(request).await {
		Ok(input) => input,
		_ => return interaction.reply(&ctx.http, "Invalid URL!").await,
	};

	let manager = songbird::get(&ctx).await.unwrap();

	let content = if let Some(call) = manager.get(guild_id) {
		let content = format!("Added **{}** to the queue!", input.metadata.title.as_deref().unwrap());

		let mut call = call.lock().await;
		call.enqueue_source(input);

		content
	} else {
		let (arc, result) = manager.join(guild_id, voice_channel_id).await;
		result.or_print("join voice channel");

		let content = format!("Now playing: **{}**", input.metadata.title.as_ref().unwrap());

		let mut call = arc.lock().await;
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
