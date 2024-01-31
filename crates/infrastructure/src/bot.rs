use std::sync::{Arc, RwLock};

use dotenvy;
use envy;
use serde::Deserialize;

use crate::storage::data_storage;

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
        Err(e) => panic!("error while parsing env variables: {}", e),
    }
}

pub async fn run(env: &Env) {
    let db = data_storage(&env.database_url).await;
    let safe_db = Arc::new(RwLock::new(db));

    void_discord_bot::run(
        safe_db,
        env.token.to_string(),
        env.prefix.to_string(),
        env.enable_whitespace,
    )
        .await;
}
