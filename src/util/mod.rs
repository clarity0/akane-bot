use serenity::{model::prelude::User};

pub fn user_handle(user: &User) -> String {
	format!("{}#{}", user.name, user.discriminator)
}