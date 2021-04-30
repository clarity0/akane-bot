use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use crate::{
	error::Error,
	models::log::{Log, LogType},
};

pub struct ErrorLog;

impl ErrorLog {
	pub async fn user_not_found(ctx: &Context, cmd_msg: &Message) -> CommandResult {
		let message = "user not found".to_string();
		Log {
			message: &message,
			log_type: LogType::Error,
		}
		.log_command(&ctx, &cmd_msg)
		.await?;
		Ok(())
	}
	pub async fn bad_user_string(ctx: &Context, cmd_msg: &Message) -> CommandResult {
		let message = "bad user string".to_string();
		Log {
			message: &message,
			log_type: LogType::Error,
		}
		.log_command(&ctx, &cmd_msg)
		.await?;
		Ok(())
	}
	pub async fn could_not_update_database(
		ctx: &Context, cmd_msg: &Message, err: Error,
	) -> CommandResult {
		let message = format!("could not update database {}", err);
		Log {
			message: &message,
			log_type: LogType::Error,
		}
		.log_command(&ctx, &cmd_msg)
		.await?;
		Ok(())
	}
}
