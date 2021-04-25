use serenity::Error;

use crate::schema::banlist;
use crate::schema::mutelist;
use crate::schema::gulaglist;

/// ### Struct for a Ban database select
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Queryable)]
pub struct Ban {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}


/// ### Struct for a Gulag database select
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Queryable)]
pub struct Gulag {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}

/// ### Struct for a Mute database select
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Queryable)]
pub struct Mute {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}

/// ### Struct for a Ban database insert
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Insertable)]
#[table_name="banlist"]
pub struct NewBan<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}
/// ### Struct for a Gulag database insert
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Insertable)]
#[table_name="gulaglist"]
pub struct NewGulag<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}
/// ### Struct for a Mute database insert
/// #### Fields
/// + `user_id` : the user id of a discord user
/// + `server_id` : the server id of a discord server
/// + `user_handle` : the tag of a discord user
/// + `date` : the date when the entry was added
#[derive(Insertable)]
#[table_name="mutelist"]
pub struct NewMute<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}

/// ### Log Message Type
/// #### Variants
/// + `Success` : for a successful command
/// + `Error(Error)` : for a failed command
///		- `Error` : diesel error type
pub enum LogType {
	Success,
	Error(Error),
}

/// ### Log Action
/// #### Variants
/// + `Add` : for an entry to the databse
/// + `Remove` : for a removal from the database
pub enum Action {
	Add,
	Remove,
}

/// ### Roles in a server
/// #### Variants
/// + `Gulag(Action)` : an interaction with the gulag role
/// + `Muted(Action)` : an interaction with the muted role
pub enum Role {
	Gulag(Action),
	Muted(Action),
}


impl ToString for Role {
/// ### To String implementation for a Role
/// + `Gulag` -> "Gulag"
/// + `Muted` -> "Muted"
	fn to_string(&self) -> String {
		match self {
			Role::Gulag(_) => "Gulag".to_string(),
			Role::Muted(_) => "Muted".to_string(),
		}
	}
}

impl Role {
/// ### Get the action associated with a role
/// + `Gulag(Add)` -> Add
/// + `Gulag(Remove)` -> Remove
/// + `Mute(Add)` -> Add
/// + `Mute(Remove)` -> Remove
	pub fn action(&self) -> Action {
		match self {
			Role::Gulag(role_action) => match role_action {
				Action::Add => Action::Add,
				Action::Remove => Action::Remove,
			}
			Role::Muted(role_action) => match role_action {
				Action::Add => Action::Add,
				Action::Remove => Action::Remove,
			}
		}
	}
}