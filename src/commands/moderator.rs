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
use chrono::{Datelike, Timelike, Utc};
use crate::database::*;
#[group]
#[commands(kick,)]
struct Moderator;
#[command]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let dc_user_id = parse_username(args.message());
	if dc_user_id.is_none() {
		msg.channel_id.say(ctx, "Failed to kick: user not found").await?;
		return Ok(());
	}
	let dc_user_id = dc_user_id.unwrap();
	let user = UserId::from(dc_user_id);
	let db_entry_user = user.to_user(&ctx).await?;
	let db_entry_user = format!("{}#{}", db_entry_user.name, db_entry_user.discriminator);
	let now = Utc::now();
	let db_entry_date = format!("{}-{:02}-{:02} {:02}:{:02}:{:02} {:?}",
		now.year(),
		now.month(),
		now.day(),
		now.hour(),
		now.minute(),
		now.second(),
		now.timezone()
	);
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
	guild.kick(&ctx, user).await?;
	let connection = establish_connection();
	add_ban(&connection, &dc_user_id.to_string(), &&db_entry_user, &db_entry_date);
	Ok(())
}