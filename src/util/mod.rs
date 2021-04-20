use std::num::ParseIntError;
use serenity::{model::prelude::User, utils::parse_username};
use chrono::{Datelike, Timelike, Utc};

pub fn utc_date_now_string() -> String {
	let now = Utc::now();
	format!("{}-{:02}-{:02} {:02}:{:02}:{:02}",
		now.year(),
		now.month(),
		now.day(),
		now.hour(),
		now.minute(),
		now.second(),
	)
}

pub fn user_handle(user: &User) -> String {
	format!("{}#{}", user.name, user.discriminator)
}

fn user_id_to_mention(user_id: &str) -> Result<String, ParseIntError> {
	if user_id.len() == 18 {
		user_id.parse::<u64>().map(|user_id| format!("<@!{}>", user_id))
	} else {
		"bad_u64".parse::<u64>().map(|_| "".to_string())
	}
}

fn string_to_mention(mention: &str) -> Result<String, ParseIntError> {
	if let None = parse_username(&mention) {
		user_id_to_mention(&mention).map(|mention| mention)
	} else {
		Ok(mention.to_string())
	}
}

pub fn string_to_user_id(mention: &str) -> Option<u64> {
	if let Ok(user_id) = string_to_mention(mention).map(|m| parse_username(m)) {
		user_id.map(|m| m)
	} else {
		None
	}
}