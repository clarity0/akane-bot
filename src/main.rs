use std::{
    env::var,
    sync::Arc,
    time::Duration,
};
use dotenv::dotenv;
use serenity::{
    prelude::*,
    framework::{
        StandardFramework,
    },
};
use tokio::time::sleep;

mod commands;
use commands::general::*;
use commands::moderator::*;
mod handler;
use handler::Handler;
mod shard_manager;
use shard_manager::ShardManagerContainer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = var("AKANE_BOT_TOKEN").expect("Bot token not found");
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

    let manager = client.shard_manager.clone();

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(60)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id,
                    runner.stage,
                    runner.latency,
                );
            }
        }
    });
    
    if let Err(msg) = client.start_shards(1).await {
        println!("Error: {:?}", msg);
    }
}