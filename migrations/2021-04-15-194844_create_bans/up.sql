-- Your SQL goes here
CREATE TABLE banlist (
	user_id TEXT PRIMARY KEY NOT NULL,
	user_handle TEXT DEFAULT NULL,
	ban_date TEXT NOT NULL
)