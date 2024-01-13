use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;

pub mod user;

pub struct BaseEventHandler;

#[async_trait]
impl EventHandler for BaseEventHandler {
    async fn message(&self, _ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        log::info!("new message from: {}, {}", new_message.author.name, new_message.content);
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        log::info!("{} is ready to use. Listening for incoming commands...", data_about_bot.user.name);
    }
}