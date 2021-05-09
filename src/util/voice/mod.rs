pub mod music;
pub mod voice_chat;

use std::sync::Arc;

use serenity::client::Context;
use songbird::Songbird;

use crate::error::Error;

struct VoiceManager;

impl VoiceManager {
	pub async fn get(ctx: &Context) -> Result<Arc<Songbird>, Error> {
		songbird::get(ctx)
			.await
			.ok_or_else(|| Error::VoiceManagerError("Could not retrieve voice manager".to_string()))
	}
}
