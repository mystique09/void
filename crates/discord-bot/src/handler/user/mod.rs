use serenity::all::{Context, EventHandler, Message};
use serenity::async_trait;

pub struct UserEventHandler;

#[async_trait]
impl EventHandler for UserEventHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        let reply = new_message.reply_mention(ctx.http, format!("Hello, {}", new_message.author.name)).await;

        if let Err(why) = reply {
            log::error!("{}", why);
        }
    }
}