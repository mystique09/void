use std::{collections::HashSet, sync::Arc};

use crate::bootstrap::{database::Database, env::Env};
use serenity::{
    framework::StandardFramework,
    http::Http,
    prelude::{RwLock, TypeMapKey},
    Client,
};

use super::{handler::Handler, DEFAULT_PREFIX};

pub struct Bot {
    pub client: Client,
}

struct SharedState;
impl TypeMapKey for SharedState {
    type Value = Arc<RwLock<Database>>;
}

impl Bot {
    pub async fn new(env: &Env) -> Self {
        let http = Http::new_with_token(&env.get_token());

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

        let client = Client::builder(&env.get_token())
            .event_handler(Handler)
            .framework(fm)
            .await
            .unwrap();

        Self { client }
    }

    pub async fn write_data(&self, db: &Database) {
        let mut data = self.client.data.write().await;
        data.insert::<SharedState>(Arc::new(RwLock::new(db.clone())));
    }

    pub async fn start(&mut self) {
        self.client.start().await.unwrap();
    }
}
