use std::{collections::HashMap, sync::Arc};

use serenity::{model::prelude::GuildId, prelude::RwLock};

use super::database::Database;
use super::env::Env;
use crate::bot::shared::{Guild, SharedEnvState, SharedGuildState, SharedUsecase, Usecase};
use crate::{bot::init::Bot, repository, usecase};

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
        let shared_guild_state: Arc<RwLock<HashMap<GuildId, Guild>>> = {
            let guilds: HashMap<GuildId, Guild> = HashMap::new();
            Arc::new(RwLock::new(guilds))
        };
        let shared_usecase = {
            let user_repo = repository::user_repository::UserRepository::new(db.clone());
            let keyword_repo = repository::keyword_repository::KeywordRepository::new(db.clone());

            Arc::new(RwLock::new(Usecase {
                user_usecase: usecase::user_usecase::UserUsecase::new(user_repo),
                keywords_usecase: usecase::auto_respond_usecase::KeywordUsecase::new(keyword_repo),
            }))
        };

        bot.write_data::<SharedEnvState>(shared_env).await;
        bot.write_data::<SharedGuildState>(shared_guild_state).await;
        bot.write_data::<SharedUsecase>(shared_usecase).await;

        Self { env, db, bot }
    }

    pub async fn start(&mut self) {
        self.bot.start().await;
    }
}
