use std::str::FromStr;
use serenity::{Error, client::Context, framework::standard::{Args, CommandResult}, model::{channel::Message, guild::{Member}, id::{RoleId, UserId}}};

use crate::models::{log::{LogType, Logging, Log}, role::{Action, RoleAction, ServerRole,}};

pub async fn role_change(role_action: RoleAction, ctx: &Context, msg: &Message, args: &Args) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			let mut user_as_member = guild.member(&ctx, user_id).await?;
			if let Some(guild_role)  = guild.role_by_name(role_action.role.to_string().as_str()) {
				if let Err(err) = guild_role_change(&role_action.action, ctx, &mut user_as_member, &guild_role.id).await {
					let log_type = LogType::Error;
					let message = format!("{} {}", role_action.log_message(&log_type, &user), err);
					Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
				} else {
					if let Err(err) = role_action.log(&user, guild) {
						let message = format!("could not update database {}", err);
						Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
					} else {
						let log_type = LogType::Success;
						let message = role_action.log_message(&log_type, &user);
						Log{message: &message, log_type}.log_command(&ctx, &msg).await?;
					}
				}
			} else {
				let message = format!("{} role not found", role_action.role.to_string());
				Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
			}
		} else {
			let message = "user not found".to_string();
			Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
		}
	} else {
		let message = "bad user format".to_string();
		Log{message: &message, log_type: LogType::Error}.log_command(&ctx, &msg).await?;
	}
	Ok(())
}

async fn guild_role_change(action: &Action, ctx: &Context, member: &mut Member, role_id: &RoleId) -> Result<(), Error> {
	match action {
		Action::Add => Ok(member.add_role(&ctx, role_id).await?),
		Action::Remove => Ok(member.remove_role(&ctx, role_id).await?),
	}
}

impl ToString for crate::models::role::ServerRole {
	fn to_string(&self) -> String {
		match self {
			ServerRole::Gulag => "Gulag".to_string(),
			ServerRole::Muted => "Muted".to_string(),
		}
	}
}