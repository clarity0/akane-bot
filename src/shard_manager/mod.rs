use std::{sync::Arc, time::Duration};

use serenity::{
	client::bridge::gateway::ShardManager,
	prelude::Mutex,
	prelude::TypeMapKey,
};
use tokio::time::sleep;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub fn shard_iterator_task(manager: Arc<Mutex<ShardManager>>) {
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
}
