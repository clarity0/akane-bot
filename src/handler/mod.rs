use serenity::{
	async_trait,
	client::{Context, EventHandler},
	model::prelude::Ready,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	/// ### Handler for connecting to a guild
	/// Outputs to stdout connection ready information
	async fn ready(&self, _: Context, ready: Ready) {
		if let Some(shard) = ready.shard {
			// Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
			//
			// This may seem unintuitive, but it models Discord's behaviour.
			println!("{} is connected on shard {}/{}!", ready.user.name, shard[0], shard[1],);
		}
	}
}
