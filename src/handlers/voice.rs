use crate::helpers::ResultHelpers;
use serenity::{async_trait, http::Http, model::id::ChannelId, prelude::Mutex};
use songbird::{Call, Event, EventContext, EventHandler};
use std::sync::Arc;

pub struct VoiceHandler {
	pub call: Arc<Mutex<Call>>,
	pub channel_id: ChannelId,
	pub http: Arc<Http>,
}

#[async_trait]
impl EventHandler for VoiceHandler {
	async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
		let option = {
			let call = self.call.lock().await;
			call.queue().current()
		};

		if let Some(handle) = option {
			let content = format!("Now playing: **{}**", handle.metadata().title.as_ref().unwrap());

			self.channel_id
				.say(&self.http, content)
				.await
				.or_print("send now playing message");
		}

		None
	}
}
