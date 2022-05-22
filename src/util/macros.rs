#[macro_export]
macro_rules! get_call {
	($ctx:expr, $interaction:expr) => {{
		let manager = songbird::get(&$ctx).await.unwrap();

		match manager.get($interaction.guild_id.unwrap()) {
			Some(call) => call,
			None => return $interaction.reply(&$ctx, replies::BOT_NOT_CONNECTED).await,
		}
	}};
}
