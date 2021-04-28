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

use crate::{database::bans::{log_ban, log_unban}, models::{log::{Log, LogType}, role::{Action, RoleAction, ServerRole}}, util::role_change};

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
            let message = format!("user not found");
			Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
		}
	} else {
		let message = format!("bad user format");
		Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
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
				let log_type = LogType::Error;
				let message = format!("could not ban user {} {}", user.tag(), err);
				Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
			} else {
				if let Err(err) = log_ban(&user, guild) {
					let message = format!("could not update database {}", err);
					Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
				} else {
					let log_type = LogType::Success;
					let message = format!("banned user {}", user.tag());
					Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
				}
			}
		} else {
            let message = format!("user not found");
			Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
		}
	} else {
		let message = format!("bad user format");
		Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
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
				let log_type = LogType::Error;
				let message = format!("could not unban user {} {}", user.tag(), err);
				Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
			} else {
				if let Err(err) = log_unban(&user, guild) {
					let message = format!("could not update database {}", err);
					Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
				} else {
					let log_type = LogType::Success;
					let message = format!("unbanned user {}", user.tag());
					Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
				}
			}
		} else {
            let message = format!("user not found");
			Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
		}
	} else {
		let message = format!("bad user format");
		Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(silence,)]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Muted,
		action: Action::Add,
	};
	role_change(role_action, &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(fema,)]
async fn gulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Gulag,
		action: Action::Add,
	};
	role_change(role_action, &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(unsilence,)]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Muted,
		action: Action::Remove,
	};
	role_change(role_action, &ctx, &msg, &args).await?;
	Ok(())
}

#[command]
#[aliases(unfema,)]
async fn ungulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Gulag,
		action: Action::Remove,
	};
	role_change(role_action, &ctx, &msg, &args).await?;
	Ok(())
}