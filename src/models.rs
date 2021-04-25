use serenity::{Error};

use crate::schema::banlist;
use crate::schema::mutelist;
use crate::schema::gulaglist;
#[derive(Queryable)]
pub struct Ban {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Mute {
	pub user_id: String,
	pub server_id: String,
	pub user_handle: String,
	pub date: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Gulag {
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

#[derive(Insertable)]
#[table_name="gulaglist"]
pub struct NewGulag<'a> {
    pub user_id: &'a str,
	pub server_id: &'a str,
    pub user_handle: &'a str,
	pub date: chrono::NaiveDateTime,
}

pub enum LogType {
	Success,
	Error(Error),
}

pub enum RoleAction {
	Add,
	Remove,
}

pub enum Role {
	Gulag(RoleAction),
	Muted(RoleAction),
}

impl ToString for Role {
	fn to_string(&self) -> String {
		match self {
			Role::Gulag(_) => "Gulag".to_string(),
			Role::Muted(_) => "Muted".to_string(),
		}
	}
}

impl Role {
	pub fn action(&self) -> RoleAction {
		match self {
			Role::Gulag(role_action) => match role_action {
				RoleAction::Add => RoleAction::Add,
				RoleAction::Remove => RoleAction::Remove,
			}
			Role::Muted(role_action) => match role_action {
				RoleAction::Add => RoleAction::Add,
				RoleAction::Remove => RoleAction::Remove,
			}
		}
	}
}