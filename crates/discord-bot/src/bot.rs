use std::collections::HashSet;
use std::sync::atomic::AtomicBool;

use serenity::all::{ActivityData, CurrentUser, GatewayIntents, OnlineStatus, UserId};
use serenity::Client;
use serenity::client::ClientBuilder;
use serenity::framework::standard::Configuration;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::TypeMapKey;

use crate::commands::prefix::general::GENERALCOMMANDS_GROUP;
use crate::commands::prefix::generator::GENERATORCOMMANDS_GROUP;
use crate::commands::prefix::HELP_CMD;
use crate::handler::BaseEventHandler;
use crate::handler::system::SystemEventHandler;
use crate::handler::user::UserEventHandler;

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

    pub async fn insert_shared_state<T: TypeMapKey>(&self, state: T::Value) {
        let mut data = self.client.data.write().await;
        data.insert::<T>(state);
    }

    pub async fn start(&mut self) {
        if let Err(why) = self.client.start_shards(2).await {
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

async fn configure_bot_options(config: &Config) -> StandardFramework {
    let bot_config = Configuration::new()
        .prefix(&config.prefix)
        .with_whitespace(config.enable_whitespace)
        .on_mention(Some(config.bot_id.id))
        .owners(config.owners.clone());

    let framework = StandardFramework::new()
        .help(&HELP_CMD)
        .group(&GENERALCOMMANDS_GROUP)
        .group(&GENERATORCOMMANDS_GROUP);
    framework.configure(bot_config);
    framework
}

async fn build_client<'a>(config: &Config, framework: StandardFramework) -> Client {
    let client = ClientBuilder::new(&config.token, config.intents)
        .framework(framework)
        .status(OnlineStatus::Online)
        .activity(ActivityData::playing("Discord"))
        .event_handler(BaseEventHandler)
        .event_handler(UserEventHandler)
        .event_handler(SystemEventHandler { is_concurrent: AtomicBool::new(false) })
        .await
        .expect("something went wrong when creating bot client");

    client
}