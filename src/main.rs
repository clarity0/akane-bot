#[macro_use]
extern crate diesel;
mod commands;
mod database;
mod env;
mod error;
mod handler;
mod models;
mod schema;
mod shard_manager;
mod util;

use commands::general::*;
use commands::moderator::*;
use env::load_env;
use handler::Handler;
use serenity::{framework::StandardFramework, prelude::*};
use shard_manager::{shard_iterator_task, ShardManagerContainer};
use std::{env::var, process, sync::Arc};

#[tokio::main]
async fn main() {
	if let Err(err) = load_env() {
		eprintln!("Error: {}", err);
		process::exit(1);
	}

	let token = var("AKANE_BOT_TOKEN").unwrap();

	let framework = StandardFramework::new()
		.configure(|c| c.prefix("!").delimiter(' '))
		.group(&GENERAL_GROUP)
		.group(&MODERATOR_GROUP);
	let mut client = Client::builder(token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Error creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
	}
	shard_iterator_task(client.shard_manager.clone());

	if let Err(msg) = client.start_shards(1).await {
		println!("Error: {:?}", msg);
	}
}
