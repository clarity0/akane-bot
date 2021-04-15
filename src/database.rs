use diesel::prelude::*;

use crate::models::{NewBan};

pub fn establish_connection() -> SqliteConnection {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", &database_url))
}

pub fn add_ban<'a>(conn: &SqliteConnection, user_id: &'a str, user_handle: &'a str, ban_date: &'a str) {
	use crate::schema::banlist;

	let new_ban = NewBan {
		user_id: user_id,
		user_handle: user_handle,
		ban_date: ban_date,
	};
	let _insert = diesel::insert_into(banlist::table).values(&new_ban).execute(conn)
		.expect("Error inserting to database");
}