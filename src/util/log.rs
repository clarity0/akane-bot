use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use crate::{
	akane_error,
	error::Error,
	models::log::{Log, LogType},
};

pub struct ErrorLog;

impl ErrorLog {
	pub async fn user_not_found(ctx: &Context, cmd_msg: &Message) -> CommandResult {
		let message = "user not found".to_string();
		akane_error!(message, ctx, cmd_msg);
		Ok(())
	}
	pub async fn bad_user_string(ctx: &Context, cmd_msg: &Message) -> CommandResult {
		let message = "bad user string".to_string();
		akane_error!(message, ctx, cmd_msg);
		Ok(())
	}
	pub async fn could_not_update_db(
		ctx: &Context, cmd_msg: &Message, err: Error,
	) -> CommandResult {
		let message = format!("could not update database {}", err);
		akane_error!(message, ctx, cmd_msg);
		Ok(())
	}
}
