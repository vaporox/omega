use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::Message;
use serenity::{async_trait, Result};

#[async_trait]
pub trait ApplicationCommandInteractionExt {
	async fn embed(&self, ctx: &Context, embed: CreateEmbed) -> Result<Message>;
	async fn reply(&self, ctx: &Context, content: impl Into<String> + Send) -> Result<Message>;
}

#[async_trait]
impl ApplicationCommandInteractionExt for ApplicationCommandInteraction {
	async fn embed(&self, ctx: &Context, mut embed: CreateEmbed) -> Result<Message> {
		let guild_id = self.guild_id.unwrap();
		let user_id = ctx.cache.current_user_id();
		let member = ctx.cache.member(guild_id, user_id).unwrap();

		if let Some(colour) = member.colour(&ctx.cache) {
			embed.colour(colour);
		}

		self.create_followup_message(ctx, |data| data.set_embed(embed)).await
	}

	async fn reply(&self, ctx: &Context, content: impl Into<String> + Send) -> Result<Message> {
		self.create_followup_message(ctx, |data| data.content(content)).await
	}
}
