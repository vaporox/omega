use crate::commands;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::voice::VoiceState;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let interaction = interaction.application_command().unwrap();

		interaction.defer(&ctx.http).await.unwrap();

		let result = match interaction.data.name.as_str() {
			"clear" => commands::clear::run(ctx, interaction).await,
			"leave" => commands::leave::run(ctx, interaction).await,
			"now-playing" => commands::now_playing::run(ctx, interaction).await,
			"play" => commands::play::run(ctx, interaction).await,
			"queue" => commands::queue::run(ctx, interaction).await,
			"remove" => commands::remove::run(ctx, interaction).await,
			"skip" => commands::skip::run(ctx, interaction).await,
			_ => return,
		};

		result.unwrap();
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		ctx.set_activity(Activity::listening("/play")).await;

		println!("{} is listening to {} guilds!", ready.user.name, ready.guilds.len());
	}

	async fn voice_state_update(&self, ctx: Context, _: Option<VoiceState>, state: VoiceState) {
		if state.user_id != ctx.cache.current_user_id() || state.channel_id.is_some() {
			return;
		}

		let manager = songbird::get(&ctx).await.unwrap();
		manager.remove(state.guild_id.unwrap()).await.ok();
	}
}
