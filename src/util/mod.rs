use serenity::{client::Context, framework::standard::CommandResult, model::{channel::Message}, utils::Color};
pub enum Log<'a> {
	Success(&'a str),
	Error(&'a str),
}

pub async fn log_command<'a>(message: Log<'a>, ctx: &Context, cmd_msg: &Message) -> CommandResult {
	match message {
		Log::Success(message) => {
			cmd_msg.channel_id.send_message(&ctx, |m| {
				m.content(format!("Success: {}",message))
				.reference_message(cmd_msg)
			}).await?;
		},
		Log::Error(message) => {
			eprintln!("ERROR {}", message);
			if let Ok(member) = cmd_msg.member(&ctx).await {
				member.user.direct_message(&ctx, |m| m.content(format!("ERROR: {}", message))).await?;
			}
			if let Some(guild) = cmd_msg.guild(&ctx).await {
				if let Some(channel_id) = guild.channel_id_from_name(&ctx, "akane-logging").await {
					if let Some(avatar_url) = cmd_msg.author.avatar_url() {
					let cmd_msg = cmd_msg.content_safe(&ctx).await;
					channel_id.send_message(&ctx, |m| m
						.embed(|e| e
							.title("Command Error")
							.description(message)
							.field("Command", cmd_msg, false)
							.thumbnail(avatar_url)
							.color(Color::RED)
					)).await?;
					}
				}
			}
		}
	}
	Ok(())
}