use std::num::ParseIntError;

use serenity::{model::prelude::User, utils::parse_username};

pub fn utc_date_now_string() -> String {
	use chrono::{Datelike, Timelike, Utc};
	let now = Utc::now();
	format!("{}-{:02}-{:02} {:02}:{:02}:{:02} {:?}",
		now.year(),
		now.month(),
		now.day(),
		now.hour(),
		now.minute(),
		now.second(),
		now.timezone()
	)
}

pub fn user_handle(user: &User) -> String {
	format!("{}#{}", user.name, user.discriminator)
}

pub fn user_id_to_mention(user_id: &str) -> Result<String, ParseIntError> {
	if user_id.len() == 18 {
		user_id.parse::<u64>().map(|user_id| format!("<@!{}>", user_id))
	} else {
		"".parse::<u64>().map(|_| "".to_string())
	}
}

pub fn string_to_mention(mention: &str) -> Result<String, ParseIntError> {
	if let None = parse_username(&mention) {
		user_id_to_mention(&mention).map(|mention| mention)
	} else {
		Ok(mention.to_string())
	}
}