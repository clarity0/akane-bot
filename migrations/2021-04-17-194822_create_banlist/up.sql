CREATE TABLE banlist (
	user_id VARCHAR(18) PRIMARY KEY,
	server_id VARCHAR(18) NOT NULL,
	user_handle VARCHAR(32) NOT NULL,
	ban_date TEXT NOT NULL
);