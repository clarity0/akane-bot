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