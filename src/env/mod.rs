pub mod checks;

use crate::error::Error;
use checks::*;
use dotenv::dotenv;

pub fn load_env() -> Result<(), Error> {
	dotenv().ok();
	check_env()
}

fn check_env() -> Result<(), Error> {
	check_akane_user_id()?;
	check_akane_log_channel_id()?;
	check_akane_token()?;
	check_database_connection()?;
	Ok(())
}
