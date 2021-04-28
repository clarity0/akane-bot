use chrono::NaiveDateTime;
#[derive(Queryable)]
pub struct Ban {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: NaiveDateTime,
}
#[derive(Queryable)]
pub struct Gulag {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Mute {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: NaiveDateTime,
}