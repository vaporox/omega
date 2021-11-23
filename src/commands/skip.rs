use crate::helpers::*;
use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
	let call = crate::get_call!(ctx, interaction);

	let content = {
		let call = call.lock().await;
		let queue = call.queue();

		match queue.current() {
			Some(current) => {
				current.stop().or_print("skip track");
				format!("Skipped **{}**!", current.metadata().title.as_ref().unwrap())
			}
			None => "There is nothing playing!".into(),
		}
	};

	interaction.reply(&ctx.http, content).await
}
