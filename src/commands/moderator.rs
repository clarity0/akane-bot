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

#[group]
#[commands(kick,)]
struct Moderator;

#[command]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let user_id = parse_username(args.message());
	if user_id.is_none() {
		msg.channel_id.say(ctx, "Failed to kick: user not found").await?;
		return Ok(());
	}
	let user_id = user_id.unwrap();
	let user = UserId::from(user_id);
	let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
	guild.kick(ctx, user).await?;
	Ok(())
}