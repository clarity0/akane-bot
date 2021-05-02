use serenity::{
	client::Context,
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::{channel::Message, id::UserId},
	utils::Color,
};
use std::str::FromStr;

use crate::{
	akane_error, akane_success,
	database::bans::{log_ban, log_unban},
	models::{
		log::{Log, LogType},
		role::{Action, RoleAction, ServerRole},
	},
	util::{log::ErrorLog, role::role_change},
};

#[group]
#[allowed_roles("Moderator")]
#[only_in(guilds)]
#[commands(ban, unban, mute, unmute, gulag, ungulag, uinfo)]
struct Moderator;

#[command]
#[aliases(dox)]
async fn uinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
		if let Ok(member) = guild.member(&ctx, user_id).await {
			msg.channel_id
				.send_message(&ctx, |m| {
					{
						m.embed(|e| {
							e.title(member.distinct())
								.description("User Info")
								.thumbnail(member.user.avatar_url().unwrap())
								.field("UserID", user_id.to_string(), false)
								.field("Nick", member.distinct(), false)
								.footer(|f| {
									f.text(&msg.author.name)
										.icon_url(&msg.author.avatar_url().unwrap())
								})
								.color(Color::FABLED_PINK)
						})
					}
					.reference_message(msg)
				})
				.await?;
		} else {
			ErrorLog::user_not_found(&ctx, &msg).await?;
		}
	} else {
		ErrorLog::bad_user_string(&ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(exile)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			if let Err(err) = guild.ban(&ctx, &user, 0).await {
				let message = format!("could not ban user {} {}", user.tag(), err);
				akane_error!(message, ctx, msg);
			} else if let Err(err) = log_ban(&user, guild) {
				ErrorLog::could_not_update_db(&ctx, &msg, err).await?;
			} else {
				let message = format!("banned user {}", user.tag());
				akane_success!(message, ctx, msg);
			}
		} else {
			ErrorLog::user_not_found(&ctx, &msg).await?;
		}
	} else {
		ErrorLog::bad_user_string(&ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(repatriate)]
async fn unban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			if let Err(err) = guild.unban(&ctx, &user).await {
				let message = format!("could not unban user {} {}", user.tag(), err);
				akane_error!(message, ctx, msg);
			} else if let Err(err) = log_unban(&user, guild) {
				ErrorLog::could_not_update_db(&ctx, &msg, err).await?;
			} else {
				let message = format!("unbanned user {}", user.tag());
				akane_success!(message, ctx, msg);
			}
		} else {
			ErrorLog::user_not_found(&ctx, &msg).await?;
		}
	} else {
		ErrorLog::bad_user_string(&ctx, &msg).await?;
	}
	Ok(())
}

#[command]
#[aliases(silence)]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Muted,
		action: Action::Add,
	};
	role_change(role_action, &ctx, &msg, &args).await
}

#[command]
#[aliases(fema)]
async fn gulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Gulag,
		action: Action::Add,
	};
	role_change(role_action, &ctx, &msg, &args).await
}

#[command]
#[aliases(unsilence)]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Muted,
		action: Action::Remove,
	};
	role_change(role_action, &ctx, &msg, &args).await
}

#[command]
#[aliases(unfema)]
async fn ungulag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let role_action = RoleAction {
		role: ServerRole::Gulag,
		action: Action::Remove,
	};
	role_change(role_action, &ctx, &msg, &args).await
}
