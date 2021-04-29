use crate::{database::establish_connection, error::Error};

use serenity::model::id::{ChannelId, UserId};
use std::str::FromStr;

pub fn check_akane_user_id() -> Result<(), Error> {
	match std::env::var("AKANE_USER_ID") {
		Ok(user_id) => {
			if let Err(err) = UserId::from_str(user_id.as_str()) {
				Err(Error::UserParseError(err))
			} else {
				Ok(())
			}
		}
		Err(err) => Err(Error::EnvError(err)),
	}
}

pub fn check_akane_log_channel_id() -> Result<(), Error> {
	match std::env::var("AKANE_LOG_CHANNEL_ID") {
		Ok(channel_id) => {
			if let Err(err) = ChannelId::from_str(channel_id.as_str()) {
				Err(Error::ChannelParseError(err))
			} else {
				Ok(())
			}
		}
		Err(err) => Err(Error::EnvError(err)),
	}
}

pub fn check_akane_token() -> Result<(), Error> {
	if let Err(err) = std::env::var("AKANE_BOT_TOKEN") {
		Err(Error::EnvError(err))
	} else {
		Ok(())
	}
}

pub fn check_database_connection() -> Result<(), Error> {
	if let Err(err) = establish_connection() {
		Err(err)
	} else {
		Ok(())
	}
}
