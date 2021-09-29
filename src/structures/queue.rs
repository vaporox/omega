use dashmap::{mapref::one::Ref, DashMap};
use serenity::{
	client::Context,
	model::id::{ChannelId, GuildId},
	prelude::TypeMapKey,
};
use std::{collections::VecDeque, sync::Arc};

pub struct QueueEntry {
	pub requests: VecDeque<String>,
	pub text_channel: ChannelId,
	pub voice_channel: ChannelId,
}

#[derive(Default)]
pub struct Queue {
	entries: DashMap<GuildId, QueueEntry>,
}

impl Queue {
	pub fn clear(&self, guild_id: GuildId) -> usize {
		self.entries
			.remove(&guild_id)
			.map_or(0, |(_, entry)| entry.requests.len())
	}

	pub fn entry(&self, guild_id: GuildId) -> Option<Ref<GuildId, QueueEntry>> {
		self.entries.get(&guild_id)
	}

	pub async fn get(ctx: &Context) -> Arc<Self> {
		let data = ctx.data.read().await;
		data.get::<Self>().unwrap().clone()
	}

	pub fn insert(&self, guild_id: GuildId, text_channel: ChannelId, voice_channel: ChannelId, request: String) {
		let mut entry = self.entries.entry(guild_id).or_insert(QueueEntry {
			requests: VecDeque::new(),
			text_channel,
			voice_channel,
		});

		entry.requests.push_back(request);
	}

	pub fn remove(&self, guild_id: GuildId, index: usize) -> Option<String> {
		let mut entry = self.entries.get_mut(&guild_id)?;
		let removed = entry.requests.remove(index);

		if entry.downgrade().requests.is_empty() {
			self.entries.remove(&guild_id);
		}

		removed
	}

	pub fn skip(&self, guild_id: GuildId) -> Option<String> {
		let mut entry = self.entries.get_mut(&guild_id)?;
		let skipped = entry.requests.pop_front();

		if entry.downgrade().requests.is_empty() {
			self.entries.remove(&guild_id);
		}

		skipped
	}
}

impl TypeMapKey for Queue {
	type Value = Arc<Self>;
}
