use crate::helpers::ResultHelpers;
use serenity::{
	async_trait,
	builder::CreateEmbed,
	http::Http,
	model::interactions::{application_command::ApplicationCommandInteraction, InteractionResponseType},
	Result,
};

#[async_trait]
pub trait InteractionHelpers {
	async fn defer_reply(&self, http: &Http) -> Result<()>;
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> Result<()>;
	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()>;
}

#[async_trait]
impl InteractionHelpers for ApplicationCommandInteraction {
	async fn defer_reply(&self, http: &Http) -> Result<()> {
		self.create_interaction_response(http, |response| {
			response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
		})
		.await
	}

	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> Result<()> {
		self.create_followup_message(http, |data| data.create_embed(f))
			.await
			.void()
	}

	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()> {
		self.create_followup_message(http, |data| data.content(content))
			.await
			.void()
	}
}
