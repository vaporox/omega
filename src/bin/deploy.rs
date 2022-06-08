use std::env;
use std::error::Error;

use serde_json::Value;
use serenity::http::Http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let application_id = env::var("APPLICATION_ID")?.parse()?;
	let token = env::var("TOKEN")?;

	let http = Http::new_with_application_id(&token, application_id);
	let deploy = env::var("DEPLOY")?;
	let data = toml::from_str::<Value>(include_str!("../../commands.toml"))?;
	let commands = data.get("commands").ok_or("toml file did not contain commands")?;

	if deploy == "GLOBAL" {
		http.create_global_application_commands(commands).await?;
	} else {
		for guild_id in deploy.split(',') {
			if let Ok(guild_id) = guild_id.parse() {
				http.create_guild_application_commands(guild_id, commands).await?;
			}
		}
	}

	Ok(())
}
