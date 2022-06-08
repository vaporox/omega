use serenity::client::Cache;
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;

pub trait MemberExt {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId>;
}

impl MemberExt for Member {
	fn voice_channel_id(&self, cache: &Cache) -> Option<ChannelId> {
		cache.guild(self.guild_id)?.voice_states.get(&self.user.id)?.channel_id
	}
}
