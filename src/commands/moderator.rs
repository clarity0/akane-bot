use serenity::{
	client::Context,
	framework::standard::{
		Args,
		CommandResult,
		macros::{command, group,},
	},
	model::{
		channel::Message,
		id::UserId
	}
};
use crate::{database::*, util::{user_handle,string_to_user_id}};

#[group]
#[allowed_roles("Moderator",)]
#[only_in(guilds)]
#[commands(ban,unban,mute,unmute,uinfo,avatar,)]
struct Moderator;

#[command]
#[aliases(exile,)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		
		if let Err(err) = guild.ban(&ctx, &user, 0).await {
			msg.channel_id.say(ctx, format!("Cannot ban user {}", user_handle(&user))).await?;
			println!("{}", err);
		} else {
			match log_ban(&user, guild) {
				Ok(ban) => println!("Banned user:\n{:?}\n------", ban),
				Err(err) => println!("Error inserting to db: {}", err),
		 	}
		}
	} else {
		msg.channel_id.say(ctx, "User not found").await?;
	}
	Ok(())
}

#[command]
#[aliases(repatriate,)]
async fn unban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		
		if let Err(err) = guild.unban(&ctx, &user).await {
			msg.channel_id.say(ctx, format!("Cannot unban user {}", user_handle(&user))).await?;
			println!("{}", err);
		} else {
			match log_unban(&user, guild) {
				Ok(ban) => println!("Unbanned user:\n{:?}\n------", ban),
				Err(err) => println!("Error deleting from db: {}", err),
		 	}
		}
	} else {
		msg.channel_id.say(ctx, "User not found").await?;
	}
	Ok(())
}

#[command]
#[aliases(silence,)]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		let mut user_as_member = guild.member(&ctx, user_id).await?;
		
		if let Some(role)  = guild.role_by_name("Muted") {
			if let Err(err) = user_as_member.add_role(&ctx, role.id).await {
				msg.channel_id.say(ctx, format!("Cannot mute user {}", user_handle(&user))).await?;
				println!("{}", err);
			} else {
				match log_mute(&user, guild) {
					Ok(mute) => {
						println!("Muted user:\n{:?}\n------", mute);
						msg.channel_id.say(ctx, format!("Muted user {}", user_handle(&user))).await?;
					},
					Err(err) => println!("Error inserting to db: {}", err),
				}
			}
		} else {
			println!("Muted role does not exist");
		}
	} else {
		msg.channel_id.say(ctx, "User not found").await?;
	}
	Ok(())
}

#[command]
#[aliases(unsilence,)]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		let mut user_as_member = guild.member(&ctx, user_id).await?;
		
		if let Some(role) = guild.role_by_name("Muted") {
			if !user_as_member.roles.contains(&role.id) {
				msg.channel_id.say(ctx, format!("User {} is already not muted", user_handle(&user))).await?;
				return Ok(())
			}
			if let Err(err) = user_as_member.remove_role(&ctx, role.id).await {
				msg.channel_id.say(ctx, format!("Cannot mute user {}", user_handle(&user))).await?;
				println!("{}", err);
			} else {
				match log_unmute(&user, guild) {
					Ok(mute) => println!("Unmuted user:\n{:?}", mute),
					Err(err) => println!("Error deleting from db: {}", err),
				}
				msg.channel_id.say(ctx, format!("Unmuted user {}", user_handle(&user))).await?;
			}
		} else {
			println!("Muted role does not exist");
		}
	} else {
		msg.channel_id.say(ctx, "User not found").await?;
	}
	Ok(())
}

#[command]
#[aliases(dox,)]
async fn uinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		let nick = user.nick_in(&ctx, guild.id).await.ok_or("Error retrieving nick")?;
		msg.channel_id.send_message(&ctx, |m| {
			m.embed(|e| e
				.title(format!("{}", user_handle(&user)))
				.description("User Info")
				.thumbnail(user.avatar_url().unwrap())
				.field("UserID", user_id, false)
				.field("Nick", nick, false)
			)	
		}).await?;
 	} else {
		msg.channel_id.say(&ctx,"User not found").await?;
	}
	Ok(())
}

#[command]
#[aliases(avi, pfp,)]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Some(user_id) = string_to_user_id(args.message()) {
		let user = UserId::from(user_id).to_user(&ctx).await?;
		let user_handle = user_handle(&user);
		if let Some(avatar_url) = user.avatar_url()  {
			msg.channel_id.send_message(&ctx, |m| {
				m.embed(|e| e
					.title(&user_handle)
					.description(format!("Avatar for {}", &user_handle))
					.image(&avatar_url)
				)	
			}).await?;
		} else {
			msg.channel_id.say(&ctx,format!("Could not retrieve avatar url for {}", user_handle)).await?;
		}
 	} else {
		msg.channel_id.say(&ctx,"User not found").await?;
	}
	Ok(())
}