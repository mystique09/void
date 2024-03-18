use std::sync::{Arc, RwLock};

use serenity::all::GatewayIntents;
use serenity::prelude::TypeMapKey;

use void_adapter::db::Database;

use crate::bot::{ClientManager, ConfigManager};
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

pub async fn run<D>(_db: Arc<RwLock<D>>, token: String, prefix: String, enable_whitespace: bool)
where
    D: Database,
{
    let system_state = Arc::new(RwLock::new(()));
    let mut config_manager =
        ConfigManager::new(token, prefix, enable_whitespace, GatewayIntents::all());
    config_manager.initialize_application_info().await;

    let mut client_manager = ClientManager::new(config_manager.get_config()).await;
    client_manager
        .insert_shared_state::<SystemState>(system_state)
        .await;
    client_manager.start().await;
}
