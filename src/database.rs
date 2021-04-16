use diesel::prelude::*;
use crate::models::{Ban, NewBan};
use crate::schema::banlist;

pub fn establish_connection() -> SqliteConnection {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", &database_url))
}

pub fn add_ban<'a>(conn: &SqliteConnection, ban: &'a Ban) {
	let new_ban = NewBan {
		user_id: &ban.user_id,
		user_handle: &ban.user_handle,
		ban_date: &ban.ban_date,
	};
	let _insert = diesel::replace_into(banlist::table).values(&new_ban).execute(conn)
		.expect("Error inserting to database");
}