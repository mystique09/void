use std::{collections::HashMap, sync::Arc};

use serenity::{model::prelude::GuildId, prelude::RwLock};

use super::database::Database;
use super::env::Env;
use crate::bot::config::{Bot, Guild, SharedGuildState, SharedState};

pub struct Application {
    pub env: Env,
    pub db: Database,
    pub bot: Bot,
}

impl Application {
    pub async fn new() -> Self {
        let env = Env::new().unwrap();
        let db = Database::new(&env).await;
        let bot = Bot::new(&env).await;

        Self { env, db, bot }
    }

    pub async fn start(&mut self) {
        let guild_cache: HashMap<GuildId, Guild> = HashMap::new();

        self.bot
            .write_data::<SharedState>(Arc::new(RwLock::new(self.db.clone())))
            .await;
        self.bot
            .write_data::<SharedGuildState>(Arc::new(RwLock::new(guild_cache)))
            .await;
        self.bot.start().await;
    }
}
