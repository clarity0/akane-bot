use chrono::Utc;
use diesel::{prelude::*, result::Error};
use serenity::model::{guild::Guild, prelude::User};
use crate::models::{Ban, Gulag, Mute, NewBan, NewGulag, NewMute, Role, Action, LogType};
use crate::schema::{banlist,mutelist, gulaglist};

/// ### Establish a connection to the database url set in .env
/// + Panics if the database url is not set
/// + Returns `Err(diesel::result::Error::ConnectionError)` if the connection failed
/// + Returns `Ok(PgConnection)` if the connection was successfully established
pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(database_url.as_str()).map(|c| c)
}
pub trait Logging {
	fn log(&self, user: &User, guild: Guild) -> Result<(), Error>;
	fn log_message(&self, log_type: LogType, user: &User) -> String;
}

impl Logging for Role {
	/// ### Wrapper function
	/// Maps a Role(Action) to its respective database modification function
	///
	/// and drops the returned struct,
	/// + `Gulag(Add)` calls log_gulag(...)
	/// + `Gulag(Remove)` calls log_ungulag(...)
	/// + `Muted(Add)` calls log_mute(...)
	/// + `Muted(Remove)` calls log_unmute(...)
	///
	/// Returns `Ok(())` if database modification was successful
	///
	/// Returns Err(Error) with the relevant error if database modification failed
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
	/// ###	Maps a Role(Action) and LogType to the relevant error or success String
	/// Read function body for the returned String
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

/// ### Logs a new ban into the database
/// + `user` : the user that has been banned
/// + `guild` : the server from which the user has been banned
/// #### Returns
/// + `Ok(Ban)` if successful where the associated value is the new entry
/// + `Err(Error)` if the insertion failed
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

/// ### Logs an unban to the database
/// + `user` : the user that has been unbanned
/// + `guild` : the server from which the user has been unbanned
/// #### Returns
/// + `Ok(Ban)` if successful where the associated value is the dropped entry
/// + `Err(Error)` if the insertion failed
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

/// ### Logs a new gulag into the database
/// + `user` : the user that has been gulagged
/// + `guild` : the server in which the user has been gulagged
/// #### Returns
/// + `Ok(Gulag)` if successful where the associated value is the new entry
/// + `Err(Error)` if the insertion failed
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

/// ### Logs an ungulag to the database
/// + `user` : the user that has been ungulagged
/// + `guild` : the server in which the user has been ungulagged
/// #### Returns
/// + `Ok(Gulag)` if successful where the associated value is the dropped entry
/// + `Err(Error)` if the insertion failed
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

/// ### Logs a new mute into the database
/// + `user` : the user that has been muted
/// + `guild` : the server in which the user has been muted
/// #### Returns
/// + `Ok(Mute)` if successful where the associated value is the new entry
/// + `Err(Error)` if the insertion failed
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

/// ### Logs an unmute to the database
/// + `user` : the user that has been unmuted
/// + `guild` : the server in which the user has been unmuted
/// #### Returns
/// + `Ok(Mute)` if successful where the associated value is the dropped entry
/// + `Err(Error)` if the insertion failed
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