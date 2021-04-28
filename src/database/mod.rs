pub mod bans;
pub mod gulags;
pub mod mutes;

use diesel::prelude::*;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(database_url.as_str()).map(|c| c)
}