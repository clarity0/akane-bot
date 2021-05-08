use serenity::{client::Context, framework::standard::{Args, CommandResult, macros::{command, group}}, model::channel::Message};

use crate::util;

#[group]
#[only_in(guilds)]
#[commands(music, deafen, undeafen, akanemute, akaneunmute, join, leave)]
struct Voice;

#[command]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::akane_deafen(&ctx, &msg, true).await
}

#[command]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::akane_deafen(&ctx, &msg, false).await
}

#[command]
async fn akanemute(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::akane_mute(&ctx, &msg, true).await
}

#[command]
async fn akaneunmute(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::akane_mute(&ctx, &msg, false).await
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::join(&ctx, &msg).await
}

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
	util::voice::leave(&ctx, &msg).await
}

#[command]
#[sub_commands(play, pause, stop,)]
async fn music(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}