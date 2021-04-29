use crate::schema::bans;
use crate::schema::gulags;
use crate::schema::mutes;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "bans"]
pub struct NewBan<'a> {
	pub user_id: &'a str,
	pub server_id: &'a str,
	pub user_handle: &'a str,
	pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "gulags"]
pub struct NewGulag<'a> {
	pub user_id: &'a str,
	pub server_id: &'a str,
	pub user_handle: &'a str,
	pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "mutes"]
pub struct NewMute<'a> {
	pub user_id: &'a str,
	pub server_id: &'a str,
	pub user_handle: &'a str,
	pub date: NaiveDateTime,
}
