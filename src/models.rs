#[derive(Queryable)]
#[derive(Debug)]
pub struct Ban {
	pub user_id: String,
	pub user_handle: String,
	pub ban_date: String,
}

use crate::schema::banlist;
#[derive(Insertable)]
#[table_name="banlist"]
pub struct NewBan<'a> {
    pub user_id: &'a str,
    pub user_handle: &'a str,
	pub ban_date: &'a str,
}