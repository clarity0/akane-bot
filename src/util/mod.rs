use std::str::FromStr;
use serenity::{Error, client::Context, framework::standard::{Args, CommandResult}, model::{channel::Message, guild::{Member}, id::{RoleId, UserId}}, utils::Color};
use crate::{database::Logging, models::{LogType, Role, RoleAction}};
pub enum Log<'a> {
	Success(&'a str),
	Error(&'a str),
}

pub async fn log_command<'a>(message: Log<'a>, ctx: &Context, cmd_msg: &Message) -> CommandResult {
	match message {
		Log::Success(message) => {
			cmd_msg.channel_id.send_message(&ctx, |m| {
				m.content(format!("Success: {}",message))
				.reference_message(cmd_msg)
			}).await?;
		},
		Log::Error(message) => {
			eprintln!("ERROR {}", message);
			if let Ok(member) = cmd_msg.member(&ctx).await {
				member.user.direct_message(&ctx, |m| m.content(format!("ERROR: {}", message))).await?;
			}
			if let Some(guild) = cmd_msg.guild(&ctx).await {
				if let Some(channel_id) = guild.channel_id_from_name(&ctx, "akane-logging").await {
					if let Some(avatar_url) = cmd_msg.author.avatar_url() {
					let cmd_msg = cmd_msg.content_safe(&ctx).await;
					channel_id.send_message(&ctx, |m| m
						.embed(|e| e
							.title("Command Error")
							.description(message)
							.field("Command", cmd_msg, false)
							.thumbnail(avatar_url)
							.color(Color::RED)
					)).await?;
					}
				}
			}
		}
	}
	Ok(())
}

pub async fn role_change(role: Role, ctx: &Context, msg: &Message, args: &Args) -> CommandResult {
	// Get UserID from arguments
	if let Ok(user_id) = UserId::from_str(args.message()) {
		if let Ok(user) = user_id.to_user(&ctx).await {
			// Retrieve guild in which command was used
			let guild = msg.guild(&ctx.cache).await.ok_or("Error retrieving guild")?;
			let mut user_as_member = guild.member(&ctx, user_id).await?;
			// Retrieve role if it exists
			if let Some(guild_role)  = guild.role_by_name(role.to_string().as_str()) {
				// Add role
				if let Err(err) = guild_role_change(role.action(), ctx, &mut user_as_member, &guild_role.id).await {
					let message = role.log_message(LogType::Error(err), &user);
					log_command(Log::Error(message.as_str()), &ctx, &msg).await?;
				} else {
					// Log successful role change
					if let Err(err) = role.log(&user, guild) {
						let message = format!("could not update database {}", err);
						log_command(Log::Error(message.as_str()), &ctx, &msg).await?;
					} else {
						let message = role.log_message(LogType::Success, &user);
						log_command(Log::Success(message.as_str()), &ctx, &msg).await?;
					}
				}
			} else {
				let message = format!("{} role not found", role.to_string());
				log_command(Log::Error(message.as_str()), &ctx, &msg).await?;
			}
		} else {
			log_command(Log::Error("user not found"), &ctx, &msg).await?;
		}
	} else {
		log_command(Log::Error("bad user format"), &ctx, &msg).await?;
	}
	Ok(())
}

async fn guild_role_change(role_action: RoleAction, ctx: &Context, member: &mut Member, role_id: &RoleId) -> Result<(), Error> {
	match role_action {
		RoleAction::Add => Ok(member.add_role(&ctx, role_id).await?),
		RoleAction::Remove => Ok(member.remove_role(&ctx, role_id).await?),
	}
}