use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use crate::models::log::{Log, LogType};

pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.unwrap();
	let guild_id = guild.id;

	if let Some(channel_id) =
		guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id)
	{
		let voice_manager =
			songbird::get(ctx).await.ok_or("Error retrieving voice manager")?.clone();

		// field 1 is the Result<...>
		if let Err(err) = voice_manager.join(guild_id, channel_id).await.1 {
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
