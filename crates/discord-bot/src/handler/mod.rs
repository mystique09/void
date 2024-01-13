use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;

pub mod user;

pub struct BaseEventHandler;

#[async_trait]
impl EventHandler for BaseEventHandler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        log::info!("Bot is ready to use, bot name is: {}", data_about_bot.user.name);
    }
}