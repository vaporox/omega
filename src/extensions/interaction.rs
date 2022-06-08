use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use crate::commands::prelude::Result;

#[async_trait]
pub trait ApplicationCommandInteractionExt {
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, ctx: &Context, f: F) -> Result;
	async fn reply<T: Into<String> + Send>(&self, ctx: &Context, content: T) -> Result;
}

#[async_trait]
impl ApplicationCommandInteractionExt for ApplicationCommandInteraction {
	async fn embed<F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send>(&self, ctx: &Context, f: F) -> Result {
		let guild_id = self.guild_id.unwrap();
		let user_id = ctx.cache.current_user_id();
		let member = ctx.cache.member(guild_id, user_id).unwrap();
		let colour = member.colour(&ctx.cache);

		self.create_followup_message(ctx, |data| {
			data.embed(|embed| {
				if let Some(colour) = colour {
					embed.colour(colour);
				}

				f(embed)
			})
		})
		.await
	}

	async fn reply<T: Into<String> + Send>(&self, ctx: &Context, content: T) -> Result {
		self.create_followup_message(ctx, |data| data.content(content)).await
	}
}
