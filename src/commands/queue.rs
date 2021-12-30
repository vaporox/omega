use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> CommandResult {
	let call = crate::get_call!(ctx, interaction);

	let guild_id = interaction.guild_id.unwrap();
	let user_id = ctx.cache.current_user_id().await;
	let member = ctx.cache.member(guild_id, user_id).await.unwrap();
	let colour = member.colour(&ctx.cache).await;

	let description = {
		let call = call.lock().await;
		let queue = call.queue().current_queue();

		replies::current_queue(&queue)
	};

	if description.is_empty() {
		interaction.reply(&ctx.http, replies::EMPTY_QUEUE).await
	} else {
		interaction
			.embed(&ctx.http, |embed| {
				if let Some(colour) = colour {
					embed.colour(colour);
				}
				embed.title("Queue").description(description)
			})
			.await
	}
}
