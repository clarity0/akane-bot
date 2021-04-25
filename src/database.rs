use chrono::Utc;
use diesel::{prelude::*, result::Error};
use serenity::model::{guild::Guild, prelude::User};
use crate::{models::{Ban, Gulag, Mute, NewBan, NewGulag, NewMute, Role, Action, LogType}};
use crate::schema::{banlist,mutelist, gulaglist};

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(database_url.as_str()).map(|c| c)
}

pub trait Logging {
	fn log(&self, user: &User, guild: Guild) -> Result<(), Error>;
	fn log_message(&self, log_type: LogType, user: &User) -> String;
}

impl Logging for Role {
	fn log(&self, user: &User, guild: Guild) -> Result<(), Error> {
		match self {
			Role::Muted(action) => {
				match action {
					Action::Add => log_mute(user, guild).map(|_| ()),
					Action::Remove => log_unmute(user, guild).map(|_| ()),
				}
			}
			Role::Gulag(action) => {
				match action {
					Action::Add => log_gulag(user, guild).map(|_| ()),
					Action::Remove => log_ungulag(user, guild).map(|_| ()),
				}
			}
		}
	}
	
	fn log_message(&self, log_type: LogType, user: &User) -> String {
		match self {
			Role::Muted(action) => {
				match action {
					Action::Add => match log_type {
						LogType::Success => format!("muted user {}", user.tag()),
						LogType::Error(err) => format!("could not mute user {} {}", user.tag(), err),
					}
					Action::Remove => match log_type {
						LogType::Success => format!("unmuted user {}", user.tag()),
						LogType::Error(err) => format!("could not mute user {} {}", user.tag(), err),
					}
				}
			}
			Role::Gulag(action) => {
				match action {
					Action::Add => match log_type {
						LogType::Success => format!("gulagged user {}", user.tag()),
						LogType::Error(err) => format!("could not gulag user {}, {}", user.tag(), err),
					}
					Action::Remove => match log_type {
						LogType::Success => format!("ungulagged user {}", user.tag()),
						LogType::Error(err) => format!("could not ungulag user {}, {}", user.tag(), err),
					}
				}
			}
		}
	}
}

pub fn log_ban(user: &User, guild: Guild) -> Result<Ban, Error> {
	if let Ok(conn) = establish_connection() {
		let new_ban = NewBan {
			user_id: &user.id.to_string(),
			server_id: &guild.id.to_string(),
			user_handle: &user.tag(),
			date: Utc::now().naive_utc(),
		};
		diesel::insert_into(banlist::table)
			.values(&new_ban)
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_unban(user: &User, guild: Guild) -> Result<Ban, Error> {
	if let Ok(conn) = establish_connection() {
		diesel::delete(banlist::table)
			.filter(banlist::user_id.eq(user.id.to_string()))
			.filter(banlist::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_mute(user: &User, guild: Guild) -> Result<Mute, Error> {
	if let Ok(conn) = establish_connection() {
		let new_mute = NewMute {
			user_id: &user.id.to_string(),
			server_id: &guild.id.to_string(),
			user_handle: &user.tag(),
			date: Utc::now().naive_utc(),
		};
		diesel::insert_into(mutelist::table)
			.values(&new_mute)
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_unmute(user: &User, guild: Guild) -> Result<Mute, Error> {
	if let Ok(conn) = establish_connection() {
		diesel::delete(mutelist::table)
			.filter(mutelist::user_id.eq(user.id.to_string()))
			.filter(mutelist::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_gulag(user: &User, guild: Guild) -> Result<Gulag, Error> {
	if let Ok(conn) = establish_connection() {
		let new_gulag = NewGulag {
			user_id: &user.id.to_string(),
			server_id: &guild.id.to_string(),
			user_handle: &user.tag(),
			date: Utc::now().naive_utc(),
		};
		diesel::insert_into(gulaglist::table)
			.values(&new_gulag)
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_ungulag(user: &User, guild: Guild) -> Result<Gulag, Error> {
	if let Ok(conn) = establish_connection() {
		diesel::delete(gulaglist::table)
			.filter(gulaglist::user_id.eq(user.id.to_string()))
			.filter(gulaglist::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}