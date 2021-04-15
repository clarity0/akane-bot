use std::sync::Arc;

use serenity::{
	client::bridge::gateway::ShardManager,
	prelude::Mutex,
	prelude::TypeMapKey,
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}