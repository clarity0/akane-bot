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
use crate::models::Ban;
use crate::util::utc_date_now_string;
#[group]
#[allowed_roles("Moderator",)]
#[only_in(guilds)]
#[commands(ban,)]
struct Moderator;

#[command]
#[aliases(gulag,)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = parse_username(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		guild.ban(&ctx, &user, 0).await?;
		let conn = establish_connection();
		let new_ban = Ban {
			user_id: user_id.to_string(),
			user_handle: format!("{}#{}", user.name, user.discriminator),
			ban_date: utc_date_now_string(),
		};
		println!("{:?}", add_ban(&conn, &new_ban));
	} else {
		msg.channel_id.say(ctx, "Failed to ban: user not found").await?;
	}
	Ok(())
}