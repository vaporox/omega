use serenity::builder::CreateEmbed;
use songbird::tracks::TrackHandle;

pub const BOT_NOT_CONNECTED: &str = "I'm not connected to a voice channel!";

pub const EMPTY_QUEUE: &str = "The queue is empty!";

pub const INVALID_POSITION: &str = "Invalid position!";

pub const LEFT_CHANNEL: &str = "Left the voice channel!";

pub const NO_VIDEO: &str = "Couldn't find a video with that search query!";

pub const USER_NOT_CONNECTED: &str = "You're not connected to a voice channel!";

pub fn cleared_queue(cleared: usize) -> String {
	match cleared {
		0 => EMPTY_QUEUE.into(),
		1 => "Removed 1 track from the queue!".into(),
		_ => format!("Removed {} tracks from the queue!", cleared),
	}
}

fn format_queue_element((index, handle): (usize, &TrackHandle)) -> String {
	let metadata = handle.metadata();
	let title = metadata.title.as_ref().unwrap();
	let url = metadata.source_url.as_ref().unwrap();
	let duration = metadata.duration.unwrap().as_secs();

	format!("{}. [{}]({}) ({})", index + 1, title, url, format_time(duration))
}

fn format_time(seconds: u64) -> String {
	if seconds >= 3600 {
		format!("{}:{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60)
	} else {
		format!("{}:{:02}", seconds / 60, seconds % 60)
	}
}

pub fn queue_embed(queue: &[TrackHandle]) -> CreateEmbed {
	let vec: Vec<_> = queue.iter().enumerate().map(format_queue_element).collect();
	let description = vec.join("\n");

	let mut embed = CreateEmbed::default();

	embed.title("Queue").description(description);

	embed
}

pub fn track_embed(handle: &TrackHandle, title: &str) -> CreateEmbed {
	let metadata = handle.metadata();
	let thumbnail = handle.metadata().thumbnail.as_ref().unwrap();

	let description = {
		let title = metadata.title.as_ref().unwrap();
		let url = metadata.source_url.as_ref().unwrap();
		let duration = metadata.duration.unwrap().as_secs();

		format!("[{}]({}) ({})", title, url, format_time(duration))
	};

	let mut embed = CreateEmbed::default();

	embed.title(title).thumbnail(thumbnail).description(description);

	embed
}

pub async fn track_embed_position(handle: &TrackHandle, title: &str) -> CreateEmbed {
	let metadata = handle.metadata();
	let thumbnail = handle.metadata().thumbnail.as_ref().unwrap();

	let description = {
		let title = metadata.title.as_ref().unwrap();
		let url = metadata.source_url.as_ref().unwrap();
		let position = handle.get_info().await.unwrap().position.as_secs();
		let duration = metadata.duration.unwrap().as_secs();

		format!(
			"[{}]({}) ({} / {})",
			title,
			url,
			format_time(position),
			format_time(duration),
		)
	};

	let mut embed = CreateEmbed::default();

	embed.title(title).thumbnail(thumbnail).description(description);

	embed
}
