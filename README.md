# akane-bot
Akane: Discord Bot written in Rust

## Commands
##### prefix is !
+ Mute
+ Unmute
+ Gulag
+ Ungulag
+ Ban
+ Unban
+ Ping
+ Latency
+ Avatar

#### Mute/Gulag/Ban
`!mute <user mention or id>`

#### Uinfo 
`!uinfo <user mention or id>`

#### Avatar 
`!avatar <user mention or id>`
___
## Host yourself  for your own server
1. Clone repository and build with cargo build
	```
	cargo build
	```
	or
	```
	cargo build --release
	```
3. Setup a postgresql instance
4. Setup your custom bot app on discord and obtain its token
5. Create a .env file with the following data
	```
	AKANE_BOT_TOKEN="YOUR BOT TOKEN"
	DATABASE_URL=postgres://your@own/database
	```
1. Obtain the diesel-cli tool and run the following command on the project directory
	```
	diesel migration run
	```
1. Setup "Muted" and "Gulag" roles with the permissions you want them to have
1. Setup an "akane-logging" channel
