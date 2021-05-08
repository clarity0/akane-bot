pub mod music;
pub mod voice_chat;

use std::sync::Arc;

use serenity::client::Context;
use songbird::Songbird;

use crate::error::Error;

struct VoiceManager;

impl VoiceManager {
	pub async fn get(ctx: &Context) -> Result<Arc<Songbird>, Error> {
		let voice_manager = songbird::get(ctx)
			.await
			.ok_or(Error::VoiceManagerError("Could not retrieve voice manager".to_string()));

		voice_manager.map(|v| v.clone())
	}
}
