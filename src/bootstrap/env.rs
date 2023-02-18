use std::env;
use tracing::info;

pub struct Env {
    pub db_url: Option<String>,
    pub token: Option<String>,
    pub guild_id: Option<String>,
}

impl Env {
    pub async fn new() -> Self {
        info!("ENVIRONMENT VARIABLES LOADED.");
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
