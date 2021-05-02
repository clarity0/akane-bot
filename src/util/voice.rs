use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use crate::models::log::{Log, LogType};

pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;

	if let Some(channel_id) =
		guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id)
	{
		let voice_manager =
			songbird::get(ctx).await.ok_or("Error retrieving voice manager")?.clone();

		// field 1 of tuple is the Result<...>
		if let Err(err) = voice_manager.join(guild.id, channel_id).await.1 {
			let message = format!("could not join voice channel {}", err);
			Log {
				message: &message,
				log_type: LogType::Error,
			}
			.log_command(&ctx, &msg)
			.await?;
		}
	} else {
		msg.author
			.direct_message(&ctx, |m| m.content("You are not connected to any voice channel"))
			.await?;
	}
	Ok(())
}

pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;

	let voice_manager = songbird::get(ctx).await.ok_or("Error retrieving voice manager")?.clone();

	if let Err(err) = voice_manager.leave(guild.id).await {
		let message = format!("could not leave voice channel {}", err);
		Log {
			message: &message,
			log_type: LogType::Error,
		}
		.log_command(&ctx, &msg)
		.await?;
	}
	Ok(())
}

pub async fn deafen(ctx: &Context, msg: &Message, deaf: bool) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;

	let voice_manager = songbird::get(ctx)
		.await
		.expect("Songbird Voice client placed in at initialisation.")
		.clone();

	if let Some(handler_lock) = voice_manager.get(guild.id) {
		let mut handler = handler_lock.lock().await;

		if let Err(e) = handler.deafen(deaf).await {
			msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await?;
		}
	} else {
		msg.reply(ctx, "Not in a voice channel").await?;
	}

	Ok(())
}

pub async fn akane_mute(ctx: &Context, msg: &Message, deaf: bool) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;

	let voice_manager = songbird::get(ctx)
		.await
		.expect("Songbird Voice client placed in at initialisation.")
		.clone();

	if let Some(handler_lock) = voice_manager.get(guild.id) {
		let mut handler = handler_lock.lock().await;

		if let Err(e) = handler.mute(deaf).await {
			msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await?;
		}
	} else {
		msg.reply(ctx, "Not in a voice channel").await?;
	}

	Ok(())
}
