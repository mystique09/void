use std::sync::{Arc, RwLock};

use serenity::all::GatewayIntents;

use void_adapter::db::Database;

use crate::bot::Void;
use crate::handler::system::SystemState;

pub mod handler;
pub mod commands;
pub mod hooks;
pub mod ui;
pub mod bot;
mod helpers;

pub async fn run<D>(_db: Arc<RwLock<D>>, token: String, prefix: String, enable_whitespace: bool)
    where
        D: Database
{
    let system_state = Arc::new(RwLock::new(()));
    let mut void = Void::new(token, prefix, enable_whitespace, GatewayIntents::all()).await;

    void.insert_shared_state::<SystemState>(system_state).await;
    void.start().await;
}