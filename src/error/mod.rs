use diesel::result::ConnectionError;
use serenity::model::misc::{ChannelIdParseError, UserIdParseError};
use std::{
	env::VarError,
	fmt::{self, Display, Formatter},
};

pub enum Error {
	DatabaseError(ConnectionError),
	EnvError(VarError),
	UserParseError(UserIdParseError),
	ChannelParseError(ChannelIdParseError),
	QueryError(diesel::result::Error),
	//VoiceJoinError(songbird::error::JoinError),
	//CommandError(serenity::framework::standard::CommandError),
	//Other(String)
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Error::DatabaseError(err) => write!(f, "{}", err),
			Error::EnvError(err) => write!(f, "{}", err),
			Error::UserParseError(err) => write!(f, "{}", err),
			Error::ChannelParseError(err) => write!(f, "{}", err),
			Error::QueryError(err) => write!(f, "{}", err),
			//Error::VoiceJoinError(err) => write!(f, "{}", err),
			//Error::CommandError(err) => write!(f, "{}", err),
			//Error::Other(err) => write!(f, "{}", err),
		}
	}
}
