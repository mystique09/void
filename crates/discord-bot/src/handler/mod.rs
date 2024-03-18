use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;

pub mod user;
pub mod system;
pub mod interaction;

pub struct BaseEventHandler;

#[async_trait]
impl EventHandler for BaseEventHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        log::info!("[SHARD #{}]: New message from: {}, {}", ctx.shard_id, new_message.author.name, new_message.content);
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        log::info!("[SHARD #{}]: {} is ready to use. Listening for incoming commands...", ctx.shard_id, data_about_bot.user.name);
    }
}