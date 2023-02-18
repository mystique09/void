use chrono::Duration;
use std::{collections::HashMap, sync::Arc};

use serenity::{
    model::prelude::{GuildId, UserId},
    prelude::RwLock,
};

use super::database::Database;
use super::env::Env;
use crate::bot::shared::{Guild, SharedBumpState, SharedGuildState};
use crate::{
    bot::{init::Bot, shared::SharedUserUsecase},
    repository, usecase,
};

pub struct Application {
    pub env: Env,
    pub db: Database,
    pub bot: Bot,
}

impl Application {
    pub async fn new() -> Self {
        let env = Env::new().await;
        let db = Database::new(&env).await;
        let bot = Bot::new(&env).await;

        Self { env, db, bot }
    }

    pub async fn start(&mut self) {
        let guild_cache: HashMap<GuildId, Guild> = HashMap::new();
        let bump_cache: Vec<(UserId, Duration)> = vec![];

        let shared_usecase = {
            let user_repo = repository::user_repository::UserRepository::new(self.db.clone());
            let user_case = usecase::user_usecase::UserUsecase::new(user_repo);
            Arc::new(RwLock::new(user_case))
        };

        self.bot
            .write_data::<SharedUserUsecase>(shared_usecase)
            .await;
        self.bot
            .write_data::<SharedGuildState>(Arc::new(RwLock::new(guild_cache)))
            .await;
        self.bot
            .write_data::<SharedBumpState>(Arc::new(RwLock::new(bump_cache)))
            .await;
        self.bot.start().await;
    }
}
