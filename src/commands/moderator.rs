use std::str::FromStr;
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
	},
	utils::Color,
};
use crate::{database::*, util::*, models::{Role,Action}};

#[group]
#[allowed_roles("Moderator",)]
#[only_in(guilds)]
#[commands(ban,unban,mute,unmute,gulag,ungulag,uinfo,)]
struct Moderator;

#[command]
#[aliases(dox,)]
async fn uinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		if let Ok(member) = guild.member(&ctx, user_id).await {
			msg.channel_id.send_message(&ctx, |m| {
				m.embed(|e| e
					.title(format!("{}",member.distinct()))
					.description("User Info")
					.thumbnail(member.user.avatar_url().unwrap())
					.field("UserID", user_id.to_string(), false)
					.field("Nick", member.distinct(), false)
					.footer(|f| f
						.text(&msg.author.name)
						.icon_url(&msg.author.avatar_url().unwrap())
					)
					.color(Color::FABLED_PINK)
				)	
			}.reference_message(msg)).await?;
		} else {
			log_command(Log::Error("user not found"), &ctx, &msg).await?;
		}
	} else {
		log_command(Log::Error("bad user format"), &ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(exile,)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			if let Err(err) = guild.ban(&ctx, &user, 0).await {
				log_command(Log::Error(format!("could not ban user {} {}", user.tag(), err).as_str()), &ctx, &msg).await?;
			} else {
				if let Err(err) = log_ban(&user, guild) {
					log_command(Log::Error(format!("could not update database {}", err).as_str()), &ctx, &msg).await?;
				} else {
					log_command(Log::Success(format!("banned user {}", user.tag()).as_str()), &ctx, &msg).await?;
				}
			}
		} else {
			log_command(Log::Error("user not found"), &ctx, &msg).await?;
		}
	} else {
		log_command(Log::Error("bad user format"), &ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(repatriate,)]
async fn unban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			if let Err(err) = guild.unban(&ctx, &user).await {
				log_command(Log::Error(format!("could not unban user {} {}", user.tag(), err).as_str()), &ctx, &msg).await?;
			} else {
				if let Err(err) = log_unban(&user, guild) {
					log_command(Log::Error(format!("could not update database {}", err).as_str()), &ctx, &msg).await?;
				} else {
					log_command(Log::Success(format!("unbanned user {}", user.tag()).as_str()), &ctx, &msg).await?;
				}
			}
		} else {
			log_command(Log::Error("user not found"), &ctx, &msg).await?;
		}
	} else {
		log_command(Log::Error("bad user format"), &ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(silence,)]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	role_change(Role::Muted(Action::Add), &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(silence,)]
async fn gulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	role_change(Role::Gulag(Action::Add), &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(unsilence,)]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	role_change(Role::Muted(Action::Remove), &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(unsilence,)]
async fn ungulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	role_change(Role::Gulag(Action::Remove), &ctx, &msg, &args).await?;
	Ok(())
}