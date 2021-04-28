use crate::models::insertable::NewBan;
use crate::models::queryable::Ban;
use crate::schema::{bans};

use chrono::Utc;
use diesel::result::Error;
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};
use crate::database::establish_connection;


pub fn log_ban(user: &User, guild: Guild) -> Result<Ban, Error> {
	if let Ok(conn) = establish_connection() {
		let new_ban = NewBan {
			user_id: &user.id.to_string(),
			server_id: &guild.id.to_string(),
			user_handle: &user.tag(),
			date: Utc::now().naive_utc(),
		};
		diesel::insert_into(bans::table)
			.values(&new_ban)
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}

pub fn log_unban(user: &User, guild: Guild) -> Result<Ban, Error> {
	if let Ok(conn) = establish_connection() {
		diesel::delete(bans::table)
			.filter(bans::user_id.eq(user.id.to_string()))
			.filter(bans::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
	} else {
		Err(Error::__Nonexhaustive)
	}
}