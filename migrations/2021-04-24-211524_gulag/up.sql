-- Your SQL goes here
CREATE TABLE gulaglist (
	user_id VARCHAR(18) PRIMARY KEY,
	server_id VARCHAR(18) NOT NULL,
	user_handle VARCHAR(32) NOT NULL,
	date TIMESTAMP NOT NULL
);