use serenity::{
	async_trait,
	builder::CreateEmbed,
	client::Context,
	http::Http,
	model::{
		guild::Member,
		id::ChannelId,
		interactions::{
			application_command::ApplicationCommandInteraction, InteractionApplicationCommandCallbackDataFlags,
		},
	},
	Result,
};

#[async_trait]
pub trait InteractionUtil {
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> Result<()>;
	async fn ephemeral<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()>;
	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()>;
}

#[async_trait]
impl InteractionUtil for ApplicationCommandInteraction {
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> Result<()> {
		self.create_interaction_response(http, |response| {
			response.interaction_response_data(|data| data.create_embed(f))
		})
		.await
	}

	async fn ephemeral<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()> {
		self.create_interaction_response(http, |response| {
			response.interaction_response_data(|data| {
				data.content(content)
					.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
			})
		})
		.await
	}

	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()> {
		self.create_interaction_response(http, |response| {
			response.interaction_response_data(|data| data.content(content))
		})
		.await
	}
}

#[async_trait]
pub trait MemberUtil {
	async fn voice_channel_id(&self, ctx: &Context) -> Option<ChannelId>;
}

#[async_trait]
impl MemberUtil for Member {
	async fn voice_channel_id(&self, ctx: &Context) -> Option<ChannelId> {
		ctx.cache
			.guild_field(self.guild_id, |guild| {
				guild.voice_states.get(&self.user.id).and_then(|state| state.channel_id)
			})
			.await
			.flatten()
	}
}
