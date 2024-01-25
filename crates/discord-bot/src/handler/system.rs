use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use serenity::all::{Context, GuildId};
use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::prelude::TypeMapKey;

use crate::helpers::system::{cpu_usage, memory_usage};

pub struct SystemState;

impl TypeMapKey for SystemState {
    type Value = Arc<RwLock<()>>;
}

pub struct SystemEventHandler {
    pub is_concurrent: AtomicBool,
}

const LOG_COOL_DOWN: u64 = 120;

#[async_trait]
impl EventHandler for SystemEventHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        log::info!("[SHARD #{}] Cache is ready.", ctx.shard_id);

        let ctx = Arc::new(ctx);

        if !self.is_concurrent.load(Ordering::Relaxed) {
            let copy = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    log_system_load(Arc::clone(&copy)).await;
                    tokio::time::sleep(Duration::from_secs(LOG_COOL_DOWN)).await;
                }
            });
            self.is_concurrent.swap(true, Ordering::Relaxed);
        }
    }
}

async fn log_system_load(ctx: Arc<Context>) {
    let cpu_load: f32 = cpu_usage().await;
    let memory_load = memory_usage().await;

    let cpu_load = format!("{:.2}%", cpu_load);
    let memory_load = format!(
        "{:.2} MB free out of {:.2} MB",
        memory_load.0, memory_load.1
    );

    log::info!(
        r#"
    [SHARD #{}] System Resource Usage
    - CPU Usage: {}
    - Memory Usage: {}
    "#,
        ctx.shard_id,
        cpu_load,
        memory_load
    );
}
