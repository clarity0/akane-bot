use crate::database::establish_connection;
use crate::error::Error;
use crate::models::insertable::NewGulag;
use crate::models::queryable::Gulag;
use crate::schema::gulags;
use chrono::Utc;
use diesel::prelude::*;
use serenity::model::{guild::Guild, prelude::User};

pub fn log_gulag(user: &User, guild: Guild) -> Result<Gulag, Error> {
	match establish_connection() {
		Ok(conn) => {
			let new_gulag = NewGulag {
				user_id: &user.id.to_string(),
				server_id: &guild.id.to_string(),
				user_handle: &user.tag(),
				date: Utc::now().naive_utc(),
			};
			diesel::insert_into(gulags::table)
				.values(&new_gulag)
				.get_result(&conn)
				.map_err(Error::QueryError)
		}
		Err(err) => Err(err),
	}
}

pub fn log_ungulag(user: &User, guild: Guild) -> Result<Gulag, Error> {
	match establish_connection() {
		Ok(conn) => diesel::delete(gulags::table)
			.filter(gulags::user_id.eq(user.id.to_string()))
			.filter(gulags::server_id.eq(guild.id.to_string()))
			.get_result(&conn)
			.map_err(Error::QueryError),
		Err(err) => Err(err),
	}
}
