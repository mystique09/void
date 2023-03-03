use std::env;
use tracing::{error, info};

#[derive(Clone)]
pub struct Env {
    db_url: Option<String>,
    token: Option<String>,
    guild_id: Option<u64>,
    channel_id: Option<u64>,
    app_id: Option<u64>,
    mode: Option<String>,
}

impl Env {
    pub async fn new() -> Self {
        match dotenv::dotenv().ok() {
            Some(dev_env) => info!("development environment variable loaded: {:?}", dev_env),
            None => {
                info!("no .env file detected, loading .env file");

                match dotenv::from_filename(".sample.env").ok() {
                    Some(prod_env) => {
                        info!("binding env variable: {:?}", prod_env);
                    }
                    None => error!("no .sample.env file detected"),
                }
            }
        }

        let database_url = env::var("DATABASE_URL").unwrap();
        let token = env::var("TOKEN").unwrap();
        let channel_id = env::var("CHANNEL_ID").unwrap();
        let guild_id = env::var("GUILD_ID").unwrap();
        let app_id = env::var("APP_ID").unwrap();
        let mode = env::var("MODE").unwrap();

        let channel_id = channel_id.parse::<u64>().unwrap();
        let guild_id = guild_id.parse::<u64>().unwrap();
        let app_id = app_id.parse::<u64>().unwrap();

        Self {
            db_url: Some(database_url),
            token: Some(token),
            guild_id: Some(guild_id),
            channel_id: Some(channel_id),
            app_id: Some(app_id),
            mode: Some(mode),
        }
    }

    pub fn get_token(&self) -> &String {
        self.token.as_ref().expect("no discord token")
    }

    pub fn get_db_url(&self) -> &String {
        self.db_url.as_ref().expect("no database url")
    }

    pub fn get_guild_id(&self) -> &u64 {
        self.guild_id.as_ref().expect("no guild id")
    }

    pub fn get_channel_id(&self) -> &u64 {
        self.channel_id.as_ref().expect("no channel id")
    }

    pub fn get_app_id(&self) -> &u64 {
        self.app_id.as_ref().expect("no app id")
    }

    pub fn get_mode(&self) -> &String {
        self.mode.as_ref().expect("no mode")
    }
}
