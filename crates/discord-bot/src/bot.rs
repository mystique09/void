use std::collections::HashSet;

use serenity::all::{CurrentUser, GatewayIntents, UserId};
use serenity::Client;
use serenity::client::ClientBuilder;
use serenity::framework::StandardFramework;
use serenity::http::Http;

pub type BotId = CurrentUser;
pub type AppInfo = Result<(HashSet<UserId>, BotId), ()>;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub bot_id: BotId,
    pub prefix: String,
    pub owners: HashSet<UserId>,
    pub enable_whitespace: bool,
    pub intents: GatewayIntents,
}

pub struct Void {
    pub config: Config,
    pub client: Client,
}

impl Void {
    pub async fn new(token: String, prefix: String, enable_whitespace: bool, intents: GatewayIntents) -> Self {
        let app_info = get_app_info(&token).await.unwrap();
        let config = Config {
            token,
            intents,
            owners: app_info.0,
            bot_id: app_info.1,
            prefix,
            enable_whitespace,
        };
        let bot_config_options = configure_bot_options(&config).await;
        let client = build_client(&config, bot_config_options).await;

        Self {
            config,
            client,
        }
    }

    pub async fn start(&mut self) {
        if let Err(why) = self.client.start().await {
            log::error!("Something went wrong while start the bot: {}", why);
        }
    }
}

async fn get_app_info(token: &str) -> AppInfo {
    let req = Http::new(token);

    let (owners, bot_id) = match req.get_current_application_info().await {
        Ok(info) => {
            let mut owners: HashSet<UserId> = HashSet::new();

            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            match req.get_current_user().await {
                Ok(bot_id) => (owners, bot_id),
                Err(err) => panic!("Couldn't get the bot ID: {}", err)
            }
        }
        Err(err) => panic!("Couldn't get application info: {}", err)
    };

    Ok((owners, bot_id))
}

async fn configure_bot_options(_config: &Config) -> StandardFramework {
    let framework = StandardFramework::new();

    framework
}

async fn build_client<'a>(config: &Config, framework: StandardFramework) -> Client {
    let client = ClientBuilder::new(&config.token, config.intents)
        .framework(framework)
        .await
        .expect("something went wrong when creating bot client");

    client
}