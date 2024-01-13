use std::sync::{Arc, RwLock};

use serenity::all::GatewayIntents;

use void_adapter::db::Database;

use crate::bot::Void;

pub mod handler;
pub mod commands;
pub mod hooks;
pub mod ui;
pub mod bot;
mod helpers;

pub async fn run<D>(_db: RwLock<Arc<D>>, token: String, prefix: String, enable_whitespace: bool)
    where
        D: Database
{
    let mut void = Void::new(token, prefix, enable_whitespace, GatewayIntents::all()).await;
    void.start().await;
}