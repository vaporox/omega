use serenity::{
	async_trait,
	builder::CreateEmbed,
	http::Http,
	model::interactions::{
		application_command::ApplicationCommandInteraction, InteractionApplicationCommandCallbackDataFlags,
	},
	Result,
};

#[async_trait]
pub trait Responses {
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, http: &Http, f: F) -> Result<()>;
	async fn ephemeral<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()>;
	async fn reply<T: ToString + Send>(&self, http: &Http, content: T) -> Result<()>;
}

#[async_trait]
impl Responses for ApplicationCommandInteraction {
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
