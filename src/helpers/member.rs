use serenity::client::Cache;
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;

pub trait MemberHelpers {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId>;
}

impl MemberHelpers for Member {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId> {
		cache.guild(self.guild_id)?.voice_states.get(&self.user.id)?.channel_id
	}
}
