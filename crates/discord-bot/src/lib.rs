use std::sync::{Arc, RwLock};
use log;
use void_adapter::db::Database;

pub mod handler;
pub mod commands;
pub mod hooks;
pub mod ui;
pub mod bot;

pub async fn run<D>(_db: RwLock<Arc<D>>, port: u16)
    where
        D: Database
{
    log::info!("running bot on port: {}", port);
}