use songbird::{tracks::TrackHandle, Call};

pub struct VoiceHandler;

impl VoiceHandler {
	pub async fn play(call: &mut Call, uri: &str) -> Option<TrackHandle> {
		let source = songbird::ytdl(uri).await.ok()?;
		Some(call.play_only_source(source))
	}
}
