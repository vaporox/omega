use serenity::{
	async_trait, client::Context, model::interactions::application_command::ApplicationCommandInteraction, Result,
};

#[async_trait]
pub trait Command {
	async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()>;
}
