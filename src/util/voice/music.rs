use serenity::{
	client::Context,
	framework::standard::{Args, CommandResult},
	model::channel::Message,
};
use songbird::ytdl;

use super::*;
use crate::util::log::ErrorLog;

pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let url = args.single::<String>()?;
	let guild = msg.guild(&ctx.cache).await.ok_or("guild")?;
	let manager = VoiceManager::get(ctx).await?;

	if let Some(call) = manager.get(guild.id) {
		let mut call_handler = call.lock().await;

		match ytdl(url.as_ref()).await {
			Ok(source) => call_handler.enqueue_source(source),
			Err(err) => {
				let err_msg = format!("{:?}", err);
				ErrorLog::other(ctx, msg, err_msg).await?;
			}
		}
	} else {
		let err_msg = "could not get call manager".to_string();
		ErrorLog::other(ctx, &msg, err_msg).await?;
	}
	Ok(())
}

pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("guild")?;

	let manager = VoiceManager::get(ctx).await?;

	if let Some(call) = manager.get(guild.id) {
		let mut call_handler = call.lock().await;
		call_handler.stop();
	} else {
		let err_msg = "could not get call manager".to_string();
		ErrorLog::other(ctx, &msg, err_msg).await?;
	}
	Ok(())
}
