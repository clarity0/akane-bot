pub enum ServerRole {
	Gulag,
	Muted,
}

pub enum Action {
	Add,
	Remove,
}

pub struct RoleAction {
	pub role: ServerRole,
	pub action: Action,
}

use super::log::{LogType, Logging};
use crate::{
	database::{gulags::*, mutes::*},
	error::Error,
};
use serenity::model::{guild::Guild, prelude::User};

trait LogMessage {
	fn message(&self, action: &Action, log_type: &LogType, user: &User) -> String;
}

impl LogMessage for ServerRole {
	fn message(&self, action: &Action, log_type: &LogType, user: &User) -> String {
		match self {
			ServerRole::Muted => match action {
				Action::Add => match log_type {
					LogType::Success => format!("muted user {}", user.tag()),
					LogType::Error => format!("could not mute user {}", user.tag()),
				},
				Action::Remove => match log_type {
					LogType::Success => format!("unmuted user {}", user.tag()),
					LogType::Error => format!("could not unmute user {}", user.tag()),
				},
			},
			ServerRole::Gulag => match action {
				Action::Add => match log_type {
					LogType::Success => format!("gulagged user {}", user.tag()),
					LogType::Error => format!("could not gulag user {}", user.tag()),
				},
				Action::Remove => match log_type {
					LogType::Success => format!("ungulagged user {}", user.tag()),
					LogType::Error => format!("could not ungulag user {}", user.tag()),
				},
			},
		}
	}
}

impl Logging for RoleAction {
	fn log(&self, user: &User, guild: Guild) -> Result<(), Error> {
		match self.role {
			ServerRole::Muted => match self.action {
				Action::Add => log_mute(user, guild).map(|_| ()),
				Action::Remove => log_unmute(user, guild).map(|_| ()),
			},
			ServerRole::Gulag => match self.action {
				Action::Add => log_gulag(user, guild).map(|_| ()),
				Action::Remove => log_ungulag(user, guild).map(|_| ()),
			},
		}
	}
	fn log_message(&self, log_type: &LogType, user: &User) -> String {
		self.role.message(&self.action, log_type, user)
	}
}
