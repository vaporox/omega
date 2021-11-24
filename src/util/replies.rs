use songbird::tracks::TrackHandle;

pub const BOT_NOT_CONNECTED: &str = "I'm not connected to a voice channel";

pub const EMPTY_QUEUE: &str = "The queue is empty!";

pub const INVALID_POSITION: &str = "Invalid position provided!";

pub const LEFT_CHANNEL: &str = "Left the voice channel!";

pub const NO_VIDEO: &str = "Couldn't find a video with that search query!";

pub const REMOVED_TITLE: &str = "Removed **1** title from the queue!";

pub const USER_NOT_CONNECTED: &str = "You're not connected to a voice channel!";

pub fn added_song(song: &str) -> String {
	format!("Added **{}** to the queue!", song)
}

pub fn cleared_queue(cleared: usize) -> String {
	format!("Removed **{}** titles from the queue!", cleared)
}

pub fn current_queue(queue: &[TrackHandle]) -> String {
	queue
		.iter()
		.enumerate()
		.map(|(i, e)| format!("`{}.` {}\n", i + 1, e.metadata().title.as_ref().unwrap()))
		.collect()
}

pub fn now_playing(song: &str) -> String {
	format!("Now playing: **{}**", song)
}

pub fn removed_song(song: &str) -> String {
	format!("Removed **{}** from the queue!", song)
}

pub fn skipped_song(song: &str) -> String {
	format!("Skipped **{}**!", song)
}
