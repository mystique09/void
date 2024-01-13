use std::sync::{Arc, RwLock};

use dotenvy;
use envy;
use serde::Deserialize;
use sqlx::PgPool;

use void_db::CloudDatabase;

#[derive(Deserialize, Debug)]
pub struct Env {
    pub database_url: String,
    pub token: String,
    pub prefix: String,
    pub enable_whitespace: bool,
}

pub fn get_env() -> Env {
    dotenvy::dotenv().expect("cannot load .env file");
    match envy::from_env::<Env>() {
        Ok(config) => config,
        Err(e) => panic!("error while parsing env variables: {}", e)
    }
}


pub async fn run(db_url: &str, env: &Env) {
    let pool: PgPool = PgPool::connect(db_url).await.unwrap();
    let db = CloudDatabase::new(pool);
    let safe_db = RwLock::new(Arc::new(db));

    void_discord_bot::run(safe_db, env.token.to_string(), env.prefix.to_string(), env.enable_whitespace).await;
}