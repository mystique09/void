use std::sync::{Arc, RwLock};

use void_adapter::db::Database;

pub mod handler;
pub mod commands;
pub mod hooks;
pub mod interface;

pub async fn run<D>(_db: RwLock<Arc<D>>, port: u16)
    where
        D: Database
{
    println!("[INFO] Running bot on port: {}", port);
}