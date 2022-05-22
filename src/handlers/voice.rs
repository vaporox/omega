use crate::util::replies;
use serenity::async_trait;
use serenity::client::Cache;
use serenity::http::Http;
use serenity::model::id::{ChannelId, GuildId};
use songbird::{Call, Event, EventContext, EventHandler};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct VoiceHandler {
	pub cache: Arc<Cache>,
	pub call: Arc<Mutex<Call>>,
	pub channel_id: ChannelId,
	pub guild_id: GuildId,
	pub http: Arc<Http>,
}

#[async_trait]
impl EventHandler for VoiceHandler {
	async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
		if let Some(handle) = self.call.lock().await.queue().current() {
			let user_id = self.cache.current_user_id();
			let member = self.cache.member(self.guild_id, user_id).unwrap();
			let colour = member.colour(&self.cache);

			self.channel_id
				.send_message(&self.http, |message| {
					message.embed(|embed| {
						if let Some(colour) = colour {
							embed.colour(colour);
						}

						replies::track_embed(&handle, "Now Playing")(embed)
					})
				})
				.await
				.unwrap();
		}

		None
	}
}
