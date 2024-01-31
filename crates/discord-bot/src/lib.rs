use std::sync::{Arc, RwLock};

use serenity::all::GatewayIntents;
use serenity::prelude::TypeMapKey;

use void_adapter::db::Database;

use crate::bot::Void;
use crate::handler::system::SystemState;

pub mod bot;
pub mod commands;
pub mod handler;
mod helpers;
pub mod hooks;
pub mod ui;

struct DataStorageState;

impl TypeMapKey for DataStorageState {
    type Value = Arc<RwLock<dyn Database>>;
}


pub async fn run<D>(db: Arc<RwLock<D>>, token: String, prefix: String, enable_whitespace: bool)
    where
        D: Database,
{
    let system_state = Arc::new(RwLock::new(()));
    let mut void = Void::new(token, prefix, enable_whitespace, GatewayIntents::all()).await;

    void.insert_shared_state::<SystemState>(system_state).await;
    void.insert_shared_state::<DataStorageState>(db).await;
    void.start().await;
}
