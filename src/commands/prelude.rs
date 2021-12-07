use serenity::{model::channel::Message, Result};

pub use crate::{handlers::VoiceHandler, helpers::*, util::replies};
pub use serenity::{client::Context, model::interactions::application_command::ApplicationCommandInteraction};
pub use songbird::{Event, TrackEvent};

pub type CommandResult = Result<Message>;
