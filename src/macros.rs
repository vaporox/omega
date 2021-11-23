#[macro_export]
macro_rules! get_call {
	($ctx:ident, $interaction:ident) => ({
		let manager = songbird::get(&$ctx).await.unwrap();

		match manager.get($interaction.guild_id.unwrap()) {
			Some(call) => call,
			None => return $interaction.reply(&$ctx.http, "I'm not in a voice channel!").await,
		}
	});
}
