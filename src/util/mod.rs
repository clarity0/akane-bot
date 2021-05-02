pub mod log;
pub mod voice;

use serenity::{
	client::Context,
	framework::standard::{Args, CommandResult},
	model::{
		channel::Message,
		guild::Member,
		id::{RoleId, UserId},
	},
	Error,
};
use std::str::FromStr;

use crate::{
	akane_error, akane_success, err_log_message,
	models::{
		log::{Log, LogType, Logging},
		role::{Action, RoleAction, ServerRole},
	},
	succ_log_message,
};

use self::log::ErrorLog;

pub async fn role_change(
	role_action: RoleAction, ctx: &Context, msg: &Message, args: &Args,
) -> CommandResult {
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			let mut user_as_member = guild.member(&ctx, user_id).await?;
			if let Some(guild_role) = guild.role_by_name(role_action.role.to_string().as_str()) {
				if let Err(err) =
					guild_role_change(&role_action.action, ctx, &mut user_as_member, &guild_role.id)
						.await
				{
					let message = err_log_message!(role_action, user, err);
					akane_error!(message, ctx, msg);
				} else if let Err(err) = role_action.log(&user, guild) {
					ErrorLog::could_not_update_db(&ctx, &msg, err).await?;
				} else {
					let message = succ_log_message!(role_action, user);
					akane_success!(message, ctx, msg);
				}
			} else {
				let message = format!("{} role not found", role_action.role.to_string());
				akane_error!(message, ctx, msg);
			}
		} else {
			ErrorLog::user_not_found(&ctx, &msg).await?;
		}
	} else {
		ErrorLog::bad_user_string(&ctx, &msg).await?;
	}
	Ok(())
}

async fn guild_role_change(
	action: &Action, ctx: &Context, member: &mut Member, role_id: &RoleId,
) -> Result<(), Error> {
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
