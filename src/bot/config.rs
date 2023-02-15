use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicBool, Arc},
};

use crate::bootstrap::{database::Database, env::Env};
use serenity::{
    framework::StandardFramework,
    http::Http,
    model::prelude::ChannelId,
    prelude::{RwLock, TypeMapKey},
    Client,
};

use super::{handler::BotHandler, DEFAULT_PREFIX};
pub struct Bot {
    pub client: Client,
}

#[derive(Debug)]
pub struct SharedState;

impl TypeMapKey for SharedState {
    type Value = Arc<RwLock<Database>>;
}

#[derive(Debug)]
pub struct SharedGuildState;

#[derive(Debug)]
pub struct Guild {
    pub channels: Vec<ChannelId>,
}

impl TypeMapKey for SharedGuildState {
    type Value = Arc<RwLock<HashMap<String, Guild>>>;
}

impl Bot {
    pub async fn new(env: &Env) -> Self {
        let http = Http::new_with_token(env.get_token());

        let (owners, _bot) = match http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();

                if let Some(team) = info.team {
                    owners.insert(team.owner_user_id);
                } else {
                    owners.insert(info.owner.id);
                }

                match http.get_current_user().await {
                    Ok(bot_id) => (owners, bot_id),
                    Err(why) => panic!("{why}"),
                }
            }
            Err(why) => panic!("{why}"),
        };

        let fm = StandardFramework::new().configure(|c| {
            c.prefix(DEFAULT_PREFIX)
                .with_whitespace(false)
                .owners(owners)
        });

        let client = Client::builder(env.get_token())
            .event_handler(BotHandler {
                is_parallelized: AtomicBool::new(false),
            })
            .framework(fm)
            .await
            .unwrap();

        Self { client }
    }

    pub async fn write_data<T: TypeMapKey>(&self, value: T::Value) {
        let mut data = self.client.data.write().await;
        data.insert::<T>(value);
    }

    pub async fn start(&mut self) {
        self.client.start().await.unwrap();
    }
}
