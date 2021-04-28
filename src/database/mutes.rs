use crate::models::insertable::NewMute;
use crate::models::queryable::Mute;
use crate::schema::{mutes};

use chrono::Utc;
use diesel::result::Error;
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};
use crate::database::establish_connection;

pub fn log_mute(user: &User, guild: Guild) -> Result<Mute, Error> {
	if let Ok(conn) = establish_connection() {
		let new_mute = NewMute {
			user_id: &user.id.to_string(),
			server_id: &guild.id.to_string(),
			user_handle: &user.tag(),
			date: Utc::now().naive_utc(),
		};
		diesel::insert_into(mutes::table)
			.values(&new_mute)
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_unmute(user: &User, guild: Guild) -> Result<Mute, Error> {
	if let Ok(conn) = establish_connection() {
		diesel::delete(mutes::table)
			.filter(mutes::user_id.eq(user.id.to_string()))
			.filter(mutes::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}