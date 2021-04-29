#[macro_use] extern crate diesel;
mod schema;
mod models;
mod database;
mod commands;
mod handler;
mod util;
mod error;
mod env;
mod shard_manager;

use std::{env::var, process, sync::Arc};
use env::load_env;
use serenity::{prelude::*, framework::StandardFramework};
use commands::general::*;
use commands::moderator::*;
use handler::Handler;
use shard_manager::{ShardManagerContainer,shard_iterator_task};

#[tokio::main]
async fn main() {
    if let Err(err) = load_env() {
        eprintln!("Error: {}", err);
        process::exit(1);
    } 
    
    let token = var("AKANE_BOT_TOKEN").unwrap();

    let framework = StandardFramework::new()
        .configure(|c | c
            .prefix("!")
            .delimiter(' '))
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