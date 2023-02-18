use std::env;
use tracing::{info, error};

pub struct Env {
    pub db_url: Option<String>,
    pub token: Option<String>,
    pub guild_id: Option<String>,
}

impl Env {
    pub async fn new() -> Self {
        match dotenv::from_filename(".sample.env").ok() {
            Some(prod_env) => info!("production environment variable loaded: {:?}", prod_env),
            None => {
                info!("no .sample.env file detected, loading .env file");
                
                match dotenv::dotenv().ok() {
                    Some(dev_env) => {
                        info!("development environment variable loaded: {:?}", dev_env);
                    },
                    None => error!("no .env file detected"),
                }
            }
        }

        let database_url = env::var("DATABASE_URL").unwrap();
        let token = env::var("TOKEN").unwrap();

        Self {
            db_url: Some(database_url),
            token: Some(token),
            guild_id: None,
        }
    }

    pub fn get_token(&self) -> &String {
        self.token.as_ref().expect("no discord token")
    }

    pub fn get_db_url(&self) -> &String {
        self.db_url.as_ref().expect("no database url")
    }

    pub fn get_guild_id(&self) -> &String {
        self.guild_id.as_ref().expect("no guild id")
    }
}
