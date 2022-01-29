use serenity::{
	async_trait,
	client::Cache,
	model::{guild::Member, id::ChannelId},
};

#[async_trait]
pub trait MemberHelpers {
	async fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId>;
}

#[async_trait]
impl MemberHelpers for Member {
	async fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId> {
		cache
			.guild_field(self.guild_id, |guild| {
				guild.voice_states.get(&self.user.id).and_then(|state| state.channel_id)
			})
			.await
			.flatten()
	}
}
