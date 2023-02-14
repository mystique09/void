use serenity::{async_trait, prelude::EventHandler};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {}
