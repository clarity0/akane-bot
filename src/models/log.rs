use crate::error::Error;
use serenity::{
	framework::standard::CommandResult,
	model::{channel::Message, guild::Guild, id::ChannelId, prelude::User},
	prelude::*,
	utils::Color,
};
use std::str::FromStr;

pub enum LogType {
	Success,
	Error,
}

pub struct Log<'a> {
	pub log_type: LogType,
	pub message: &'a String,
}

impl<'a> Log<'a> {
	pub async fn log_command(&self, ctx: &Context, cmd_msg: &Message) -> CommandResult {
		match &self.log_type {
			LogType::Success => {
				cmd_msg
					.channel_id
					.send_message(&ctx, |m| {
						m.content(format!("Success: {}", self.message)).reference_message(cmd_msg)
					})
					.await?;
			}
			LogType::Error => {
				let err_message = format!("Error: {}", self.message);
				// Output to stderr
				eprintln!("{}", err_message);

				// DM command invoker
				if let Ok(member) = cmd_msg.member(&ctx).await {
					member.user.direct_message(&ctx, |m| m.content(&err_message)).await?;
				}

				// Log in logging channel
				let log_channel_id =
					ChannelId::from_str(std::env::var("AKANE_LOG_CHANNEL_ID").unwrap().as_str())
						.unwrap();

				if let Some(avatar_url) = cmd_msg.author.avatar_url() {
					let cmd_msg = cmd_msg.content_safe(&ctx).await;
					log_channel_id
						.send_message(&ctx, |m| {
							m.embed(|e| {
								e.title("Command Error")
									.description(&err_message)
									.field("Command", cmd_msg, false)
									.thumbnail(avatar_url)
									.color(Color::RED)
							})
						})
						.await?;
				}
			}
		}
		Ok(())
	}
}

pub trait Logging {
	fn log(&self, user: &User, guild: Guild) -> Result<(), Error>;
	fn log_message(&self, log_type: &LogType, user: &User) -> String;
}
