use diesel::prelude::*;
use crate::models::{Ban, NewBan};
use crate::schema::banlist;

pub fn establish_connection() -> PgConnection {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", &database_url))
}

pub fn add_ban<'a>(conn: &PgConnection, ban: &'a Ban) -> Ban {
	let new_ban = NewBan {
		user_id: &ban.user_id,
		user_handle: &ban.user_handle,
		ban_date: &ban.ban_date,
	};
	diesel::insert_into(banlist::table)
		.values(&new_ban)
		.get_result(conn)
		.expect("Error inserting to database")
}