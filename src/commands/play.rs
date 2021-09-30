use crate::structures::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let guild_id = interaction.guild_id.unwrap();

	let voice_channel_id = match interaction.member.as_ref().unwrap().voice_channel_id(&ctx).await {
		Some(channel_id) => channel_id,
		_ => return interaction.reply(&ctx.http, "You're not in a voice channel!").await,
	};

	let option = interaction.data.options.get(0).unwrap();
	let request = option.value.as_ref().unwrap().as_str().unwrap();

	let queue = Queue::get(&ctx).await;
	queue.insert(guild_id, interaction.channel_id, voice_channel_id, request.into());

	let content = if queue.entry(guild_id).unwrap().requests.len() == 1 {
		let manager = songbird::get(&ctx).await.unwrap();

		let (call, result) = manager.join(guild_id, voice_channel_id).await;

		if let Err(error) = result {
			eprintln!("Error joining voice channel: {}", error);
		};

		let mut call = call.lock().await;

		if let Err(error) = call.deafen(true).await {
			eprintln!("Error deafening the bot: {}", error);
		}

		match VoiceHandler::play(&mut call, request).await {
			Some(_) => format!("Now playing: **{}**", request),
			None => "Not a valid URL!".into(),
		}
	} else {
		format!("Added to the queue: **{}**", request)
	};

	interaction.reply(&ctx.http, content).await
}
