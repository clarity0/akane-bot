use serenity::{
	client::Context,
	framework::standard::{
		macros::{command, group},
		CommandResult,
	},
	model::channel::Message,
};

#[group]
#[only_in(guilds)]
#[commands(deafen)]
struct Voice;

#[command]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;

	let voice_manager = songbird::get(ctx)
		.await
		.expect("Songbird Voice client placed in at initialisation.")
		.clone();

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
