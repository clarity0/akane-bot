pub mod bans;
pub mod gulags;
pub mod mutes;

use diesel::prelude::*;

use crate::error::Error;

pub fn establish_connection() -> Result<PgConnection, Error> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(database_url.as_str()).map_err(Error::DatabaseError)
}
