use serenity::{
	client::{
        Context,
        bridge::gateway::ShardId,
    },
	framework::standard::{
		Args, 
		CommandResult,
		macros::{command, group,}
	},
	model::channel::Message,
	utils::{
		ContentSafeOptions,
		content_safe,
	},
};

use crate::shard_manager;
use shard_manager::ShardManagerContainer;

#[group]
#[commands(ping, say, latency)]
struct General;

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Replace unsafe content with safe content
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
    msg.channel_id.say(&ctx.http, &content).await?;
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;
    Ok(())
}

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