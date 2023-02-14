use super::database::Database;
use super::env::Env;
use crate::bot::config::Bot;

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
        self.bot.write_data(&self.db).await;
        self.bot.start().await;
    }
}
