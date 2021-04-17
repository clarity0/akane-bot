use diesel::{prelude::*, result::Error};
use serenity::model::{guild::Guild, prelude::User};
use crate::{models::{Ban, NewBan}, util::user_handle};
use crate::schema::banlist;
use crate::util::utc_date_now_string;

pub fn establish_connection() -> PgConnection {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", &database_url))
}

pub fn log_ban<'a>(user: User, guild: Guild) -> Result<Ban, Error> {
	let conn = establish_connection();
	let new_ban = NewBan {
		user_id: &user.id.to_string(),
		server_id: &guild.id.to_string(),
		user_handle: &user_handle(user),
		ban_date: &utc_date_now_string(),
	};
	diesel::insert_into(banlist::table)
		.values(&new_ban)
		.get_result(&conn)
}