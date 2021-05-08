use diesel::result::ConnectionError;
use serenity::model::misc::{ChannelIdParseError, UserIdParseError};
use std::{
	env::VarError,
	fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum Error {
	DatabaseError(ConnectionError),
	EnvError(VarError),
	UserParseError(UserIdParseError),
	ChannelParseError(ChannelIdParseError),
	QueryError(diesel::result::Error),
	VoiceManagerError(String),
	//VoiceJoinError(songbird::error::JoinError),
	//CommandError(serenity::framework::standard::CommandError),
	//Other(String)
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		"akane error"
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Error::DatabaseError(err) => write!(f, "{}", err),
			Error::EnvError(err) => write!(f, "{}", err),
			Error::UserParseError(err) => write!(f, "{}", err),
			Error::ChannelParseError(err) => write!(f, "{}", err),
			Error::QueryError(err) => write!(f, "{}", err),
			Error::VoiceManagerError(err) => write!(f, "{}", err),
			//Error::VoiceJoinError(err) => write!(f, "{}", err),
			//Error::CommandError(err) => write!(f, "{}", err),
			//Error::Other(err) => write!(f, "{}", err),
		}
	}
}

#[macro_export]
macro_rules! akane_error {
	($message:expr, $ctx:expr, $msg:expr) => {
		Log {
			message: &$message,
			log_type: LogType::Error,
		}
		.log_command(&$ctx, &$msg)
		.await?;
	};
}

#[macro_export]
macro_rules! akane_success {
	($message:expr, $ctx:expr, $msg:expr) => {
		Log {
			message: &$message,
			log_type: LogType::Success,
		}
		.log_command(&$ctx, &$msg)
		.await?;
	};
}

#[macro_export]
macro_rules! err_log_message {
	($role_action:expr, $user:expr, $err:expr) => {
		format!("{} {}", $role_action.log_message(&LogType::Error, &$user), $err)
	};
}

#[macro_export]
macro_rules! succ_log_message {
	($role_action:expr, $user:expr) => {
		$role_action.log_message(&LogType::Success, &$user)
	};
}
