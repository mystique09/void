use chrono::Duration;
use std::{collections::HashMap, sync::Arc};

use serenity::{
    model::prelude::{GuildId, UserId},
    prelude::RwLock,
};

use super::database::Database;
use super::env::Env;
use crate::bot::shared::{
    Guild, SharedBumpState, SharedEnvState, SharedGuildState, SharedKeywordUsecase,
    SharedKeywordsState,
};
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

        let shared_env = Arc::new(RwLock::new(env.clone()));
        let guild_cache = {
            let guilds: HashMap<GuildId, Guild> = HashMap::new();
            Arc::new(RwLock::new(guilds))
        };
        let bump_cache = {
            let bumps: Vec<(UserId, Duration)> = vec![];
            Arc::new(RwLock::new(bumps))
        };
        let shared_keyword_state = {
            let keywords: Vec<crate::domain::auto_respond::Keyword> = vec![];
            Arc::new(RwLock::new(keywords))
        };
        let shared_user_usecase = {
            let user_repo = repository::user_repository::UserRepository::new(db.clone());
            let user_case = usecase::user_usecase::UserUsecase::new(user_repo);
            Arc::new(RwLock::new(user_case))
        };
        let shared_keyword_usecase = {
            let keyword_repo = repository::keyword_repository::KeywordRepository::new(db.clone());
            let keyword_usecase = usecase::keyword_usecase::KeywordUsecase::new(keyword_repo);
            Arc::new(RwLock::new(keyword_usecase))
        };

        bot.write_data::<SharedEnvState>(shared_env).await;
        bot.write_data::<SharedGuildState>(guild_cache).await;
        bot.write_data::<SharedKeywordsState>(shared_keyword_state)
            .await;
        bot.write_data::<SharedBumpState>(bump_cache).await;
        bot.write_data::<SharedUserUsecase>(shared_user_usecase)
            .await;
        bot.write_data::<SharedKeywordUsecase>(shared_keyword_usecase)
            .await;

        Self { env, db, bot }
    }

    pub async fn start(&mut self) {
        self.bot.start().await;
    }
}
