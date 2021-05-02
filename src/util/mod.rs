use crate::models::role::ServerRole;

pub mod log;
pub mod role;
pub mod voice;

impl ToString for crate::models::role::ServerRole {
	fn to_string(&self) -> String {
		match self {
			ServerRole::Gulag => "Gulag".to_string(),
			ServerRole::Muted => "Muted".to_string(),
		}
	}
}
