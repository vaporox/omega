use crate::commands::prelude::CommandResult;
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
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> CommandResult;
	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> CommandResult;
}

#[async_trait]
impl InteractionHelpers for ApplicationCommandInteraction {
	async fn defer_reply(&self, http: &Http) -> Result<()> {
		self.create_interaction_response(http, |response| {
			response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
		})
		.await
	}

	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> CommandResult {
		self.create_followup_message(http, |data| data.create_embed(f)).await
	}

	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> CommandResult {
		self.create_followup_message(http, |data| data.content(content)).await
	}
}
