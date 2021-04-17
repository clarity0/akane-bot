use serenity::{
	client::Context,
	framework::standard::{
		Args,
		CommandResult,
		macros::{command, group,},
	},
	model::{
		channel::Message,
		id::UserId,
	},
	utils::parse_username
};
use crate::database::*;
use crate::util::user_handle;
#[group]
#[allowed_roles("Moderator",)]
#[only_in(guilds)]
#[commands(ban,)]
struct Moderator;

#[command]
#[aliases(exile,)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = parse_username(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		
		if let Err(err) = guild.ban(&ctx, &user, 0).await {
			msg.channel_id.say(ctx, format!("Cannot ban user {}", user_handle(user))).await?;
			println!("{}", err);
		} else {
			match log_ban(user, guild) {
				Ok(ban) => println!("Banned user:\n{:?}\n------", ban),
				Err(err) => println!("Error inserting to db: {}", err),
		 	}
		}
	} else {
		msg.channel_id.say(ctx, "Failed to ban: user not found").await?;
	}
	Ok(())
}