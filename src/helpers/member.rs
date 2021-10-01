use serenity::{
	async_trait,
	client::Context,
	model::{guild::Member, id::ChannelId},
};

#[async_trait]
pub trait MemberHelpers {
	async fn voice_channel_id(&self, ctx: &Context) -> Option<ChannelId>;
}

#[async_trait]
impl MemberHelpers for Member {
	async fn voice_channel_id(&self, ctx: &Context) -> Option<ChannelId> {
		ctx.cache
			.guild_field(self.guild_id, |guild| {
				guild.voice_states.get(&self.user.id).and_then(|state| state.channel_id)
			})
			.await
			.flatten()
	}
}
