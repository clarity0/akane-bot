use crate::schema::banlist;
use crate::schema::mutelist;
#[derive(Queryable)]
#[derive(Debug)]
pub struct Ban {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}
#[derive(Queryable)]
#[derive(Debug)]
pub struct Mute {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="banlist"]
pub struct NewBan<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="mutelist"]
pub struct NewMute<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}