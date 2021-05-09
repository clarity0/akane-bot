use serenity::{
	client::Context,
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::channel::Message,
};

use crate::util::voice::{music, voice_chat};

#[group]
#[only_in(guilds)]
#[commands(play, stop, vc)]
struct Voice;

#[command]
#[sub_commands(join, leave, mute, unmute, deafen, undeafen)]
async fn vc() -> CommandResult {
	Ok(())
}

#[command]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::deafen(&ctx, &msg, true).await
}

#[command]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::deafen(&ctx, &msg, false).await
}

#[command]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::mute(&ctx, &msg, true).await
}

#[command]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::mute(&ctx, &msg, false).await
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::join(&ctx, &msg).await?;
	voice_chat::deafen(&ctx, &msg, true).await
}

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
	voice_chat::leave(&ctx, &msg).await
}

#[command]
#[aliases(p)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	music::play(ctx, msg, args).await?;
	Ok(())
}

#[command]
#[aliases(s)]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	music::stop(ctx, msg).await?;
	Ok(())
}
