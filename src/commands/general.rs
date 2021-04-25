use std::str::FromStr;
use serenity::{
    client::{
        Context,
        bridge::gateway::ShardId,
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::{
            command, group,
        }
    },
    model::{
        channel::Message,
        id::UserId
    },
    utils::Color,
};
use crate::{shard_manager, util::{log_command, Log,}};
use shard_manager::ShardManagerContainer;

#[group]
#[only_in(guilds)]
#[commands(ping, latency, avatar,)]
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
                    msg.reply(ctx, "Try again in 30 seconds").await?;
                }
            } else {
                msg.reply(ctx,  "Error: No shard found").await?;
            }
        }
        None => {
            msg.reply(ctx, "Error: There was a problem getting the shard manager").await?;
        }
    }
    Ok(())
}

/// ### Avatar command
/// #### Aliases
/// + avi
/// + pfp
///
/// Replies to the command message with an embed of the argument user's avatar
#[command]
#[aliases(avi, pfp,)]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
            let user_tag = user.tag();
			if let Some(avatar_url) = user.avatar_url()  {
				msg.channel_id.send_message(&ctx, |m| {
					m.embed(|e| e
						.title(&user_tag)
						.description(format!("Avatar for {}", &user_tag))
						.image(&avatar_url)
                        .color(Color::FABLED_PINK)
					)	
				}.reference_message(msg)).await?;
			} else {
				log_command(Log::Success(format!("could not retrieve avatar for user {}", &user_tag).as_str()), &ctx, &msg).await?;
			}
		} else {
			log_command(Log::Error("user not found"), &ctx, &msg).await?;
		}
	} else {
		log_command(Log::Error("bad user format"), &ctx, &msg).await?;
	}
	Ok(())
}