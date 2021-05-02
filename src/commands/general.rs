use crate::{akane_error, akane_success, models::log::{Log, LogType}, shard_manager, util::log::ErrorLog};
use serenity::{
	client::{bridge::gateway::ShardId, Context},
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::{channel::Message, id::UserId},
	utils::Color,
};
use shard_manager::ShardManagerContainer;
use std::str::FromStr;

#[group]
#[only_in(guilds)]
#[commands(ping, latency, avatar)]
struct General;

/// ### Ping command
/// Replies to the command message with "pong!"
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx.http, "pong!").await?;
	Ok(())
}

/// ### Latency command
/// Replies to the command message with the websocket latency
#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
	let data = &ctx.data.read().await;

	match data.get::<ShardManagerContainer>() {
		Some(shard_manager) => {
			let manager = shard_manager.lock().await;
			let runners = manager.runners.lock().await;

			if let Some(runner) = runners.get(&ShardId(ctx.shard_id)) {
				if let Some(latency) = runner.latency {
					let latency = latency.as_millis();
					msg.reply(ctx, &format!("My latency is {} ms", latency)).await?;
				} else {
					let message = "wait until first socket poll".to_string();
					akane_error!(message, ctx, msg);
				}
			} else {
				let message = "no shard found".to_string();
				akane_error!(message, ctx, msg);
			}
		}
		None => {
			let message = "could not retrieve shard manager".to_string();
			akane_error!(message, ctx, msg);
		}
	}
	Ok(())
}

#[command]
#[aliases(avi, pfp)]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let user_tag = user.tag();
			if let Some(avatar_url) = user.avatar_url() {
				msg.channel_id
					.send_message(&ctx, |m| {
						{
							m.embed(|e| {
								e.title(&user_tag)
									.description(format!("Avatar for {}", &user_tag))
									.image(&avatar_url)
									.color(Color::FABLED_PINK)
							})
						}
						.reference_message(msg)
					})
					.await?;
			} else {
				let message = format!("could not retrieve avatar for {}", &user_tag);
				akane_error!(message, ctx, msg);
			}
		} else {
			ErrorLog::user_not_found(&ctx, &msg).await?;
		}
	} else {
		ErrorLog::bad_user_string(&ctx, &msg).await?;
	}
	Ok(())
}
