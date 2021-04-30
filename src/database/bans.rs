use crate::database::establish_connection;
use crate::error::Error;
use crate::models::insertable::NewBan;
use crate::models::queryable::Ban;
use crate::schema::bans;
use chrono::Utc;
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};

pub fn log_ban(user: &User, guild: Guild) -> Result<Ban, Error> {
	match establish_connection() {
		Ok(conn) => {
			let new_ban = NewBan {
				user_id: &user.id.to_string(),
				server_id: &guild.id.to_string(),
				user_handle: &user.tag(),
				date: Utc::now().naive_utc(),
			};
			diesel::insert_into(bans::table)
				.values(&new_ban)
				.get_result(&conn)
				.map_err(Error::QueryError)
		}
		Err(err) => Err(err),
	}
}

pub fn log_unban(user: &User, guild: Guild) -> Result<Ban, Error> {
	match establish_connection() {
		Ok(conn) => diesel::delete(bans::table)
			.filter(bans::user_id.eq(user.id.to_string()))
			.filter(bans::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
			.map_err(Error::QueryError),
		Err(err) => Err(err),
	}
}
