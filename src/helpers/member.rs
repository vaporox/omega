use serenity::client::Cache;
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;

pub trait MemberHelpers {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId>;
}

impl MemberHelpers for Member {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId> {
		cache
			.guild_field(self.guild_id, |guild| {
				guild.voice_states.get(&self.user.id).and_then(|state| state.channel_id)
			})
			.flatten()
	}
}
