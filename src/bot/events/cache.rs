use chrono::Utc;
use serenity::{
    model::prelude::{Activity, ChannelId, GuildId},
    prelude::Context,
};
use std::{sync::Arc, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use tracing::error;

use crate::bot::{
    shared::{SharedEnvState, SharedGuildState},
    LOG_COOLDOWN,
};

pub async fn bind_guilds(ctx: Arc<Context>, guilds: Vec<GuildId>) {
    tokio::spawn(async move {
        let guilds_cache = {
            let data = ctx.data.read().await;
            data.get::<SharedGuildState>().unwrap().clone()
        };

        for guild_id in guilds.iter() {
            let mut guild_cache_lock = guilds_cache.write().await;
            let guild = ctx.cache.guild(guild_id).unwrap();
            let channels = guild
                .channels
                .into_iter()
                .map(|c| (c.1.to_string(), c.0))
                .collect();

            guild_cache_lock.insert(
                guild.id,
                crate::bot::shared::Guild {
                    bumps: vec![],
                    keywords: vec![],
                    channels,
                },
            );
        }
    });
}

pub async fn log_system(ctx: Arc<Context>) {
    tokio::spawn(async move {
        loop {
            log_system_load(Arc::clone(&ctx)).await;
            tokio::time::sleep(Duration::from_secs(LOG_COOLDOWN)).await;
        }
    });
}

pub async fn set_bot_status(ctx: Arc<Context>) {
    tokio::spawn(async move {
        loop {
            set_status_current_time(Arc::clone(&ctx)).await;
            tokio::time::sleep(Duration::from_secs(LOG_COOLDOWN / 2)).await;
        }
    });
}

async fn log_system_load(ctx: Arc<Context>) {
    let data = ctx
        .data
        .read()
        .await
        .get::<SharedEnvState>()
        .unwrap()
        .clone();

    let mut sys = System::default();
    sys.refresh_all();

    let cpu_load = match sys.cpus().get(0) {
        Some(cpu) => format!("{:.2}%", cpu.cpu_usage() * 10.0),
        None => "not available".to_string(),
    };
    let mem_use = format!(
        "{:.2} MB Free out of {:.2}",
        sys.free_memory() as f32 / 1000.0,
        sys.total_memory() as f32 / 1000.0
    );

    let data_lock = data.write().await;
    let channel_id = data_lock.get_channel_id();

    // We can use ChannelId from the env variable you set(IMHO, it should be a logs text channel), directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(*channel_id)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("System Resource Load")
                    .field("CPU Load Average", cpu_load, false)
                    .field("Memory Usage", mem_use, false)
            })
        })
        .await;
    if let Err(why) = message {
        error!("Error sending message: {:?}", why);
    };
}

async fn set_status_current_time(ctx: Arc<Context>) {
    let current_time = Utc::now();
    let guild_counts = ctx.cache.guild_count();

    let activity = Activity::playing(format!(
        "Waiting for commands in {} guilds.\nCurrent Time: {}",
        guild_counts,
        current_time.to_rfc2822()
    ));

    ctx.set_activity(activity).await;
}
