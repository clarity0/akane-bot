use serenity::{client::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::Message};

#[group]
#[only_in(guilds)]
#[commands(deafen, join)]
struct Voice;

#[command]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.ok_or("could not retrieve guild")?;

    let voice_manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

	if let Some(handler_lock) = voice_manager.get(guild.id) {
		let mut handler = handler_lock.lock().await;

    	if handler.is_deaf() {
      		msg.channel_id.say(&ctx.http, "Already deafened").await?;
    	} else if let Err(e) = handler.deafen(true).await {
            msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await?;
    	}

        msg.channel_id.say(&ctx.http, "Deafened").await?;
    } else {
		msg.reply(ctx, "Not in a voice channel").await?;
	}

    Ok(())
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}