pub mod bans;
pub mod gulags;
pub mod mutes;

use diesel::prelude::*;

use crate::error::Error;

pub fn establish_connection() -> Result<PgConnection, Error> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(database_url.as_str()).map_err(Error::DatabaseError)
}

#[macro_export]
macro_rules! log_add {
	($fn_name: ident, $db_table:ident, $struct_type:ty, $new_type:tt) => {
		pub fn $fn_name(user: &User, guild: Guild) -> Result<$struct_type, crate::error::Error> {
			match establish_connection() {
				Ok(conn) => {
					use chrono::Utc;
					let new: $new_type = $new_type {
						user_id: &user.id.to_string(),
						server_id: &guild.id.to_string(),
						user_handle: &user.tag(),
						date: Utc::now().naive_utc(),
					};
					diesel::insert_into($db_table::table)
						.values(&new)
						.get_result(&conn)
						.map_err(Error::QueryError)
				}
				Err(err) => Err(err),
			}
		}	
	};
}

#[macro_export]
macro_rules! log_remove {
	($fn_name: ident, $db_table:ident, $struct_type:ty) => {
		pub fn $fn_name(user: &User, guild: Guild) -> Result<$struct_type, crate::error::Error> {
			match establish_connection() {
				Ok(conn) => diesel::delete($db_table::table)
					.filter($db_table::user_id.eq(user.id.to_string()))
					.filter($db_table::server_id.eq(guild.id.to_string()))
					.get_result(&conn)
					.map_err(Error::QueryError),
				Err(err) => Err(err),
			}
		}	
	};
}
