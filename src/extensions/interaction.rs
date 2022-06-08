use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use crate::commands::prelude::Result;

#[async_trait]
pub trait ApplicationCommandInteractionExt {
	async fn embed(&self, ctx: &Context, embed: CreateEmbed) -> Result;
	async fn reply(&self, ctx: &Context, content: impl Into<String> + Send) -> Result;
}

#[async_trait]
impl ApplicationCommandInteractionExt for ApplicationCommandInteraction {
	async fn embed(&self, ctx: &Context, mut embed: CreateEmbed) -> Result {
		let guild_id = self.guild_id.unwrap();
		let user_id = ctx.cache.current_user_id();
		let member = ctx.cache.member(guild_id, user_id).unwrap();

		if let Some(colour) = member.colour(&ctx.cache) {
			embed.colour(colour);
		}

		self.create_followup_message(ctx, |data| data.set_embed(embed)).await
	}

	async fn reply(&self, ctx: &Context, content: impl Into<String> + Send) -> Result {
		self.create_followup_message(ctx, |data| data.content(content)).await
	}
}
