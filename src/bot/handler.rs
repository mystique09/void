use chrono::Utc;
use serenity::{
    async_trait,
    model::{
        prelude::{Activity, ChannelId, GuildId, Ready},
        user::OnlineStatus,
    },
    prelude::{Context, EventHandler},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

pub struct BotHandler {
    pub is_parallelized: AtomicBool,
}

// cooldown for sending system resource
// let's set this to 2 minutes
const COOLDOWN: u64 = 120;

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity: Option<Activity> = Some(Activity::playing("NeoVim"));

        ctx.set_presence(activity, OnlineStatus::Online).await;
        println!("{} is now open.", &ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfuly.");

        let ctx = Arc::new(ctx);

        if !self.is_parallelized.load(Ordering::Relaxed) {
            let ctxcpy1 = Arc::clone(&ctx);

            tokio::spawn(async move {
                loop {
                    log_system_load(Arc::clone(&ctxcpy1)).await;
                    tokio::time::sleep(Duration::from_secs(COOLDOWN)).await;
                }
            });

            let ctxcpy2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_status_current_time(Arc::clone(&ctxcpy2)).await;
                    tokio::time::sleep(Duration::from_secs(COOLDOWN / 2)).await;
                }
            });

            self.is_parallelized.swap(true, Ordering::Relaxed);
        }
    }
}

async fn log_system_load(ctx: Arc<Context>) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(920359624752893952)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("System Resource Load")
                    .field(
                        "CPU Load Average",
                        format!("{:.2}%", cpu_load.one * 10.0),
                        false,
                    )
                    .field(
                        "Memory Usage",
                        format!(
                            "{:.2} MB Free out of {:.2} MB",
                            mem_use.free as f32 / 1000.0,
                            mem_use.total as f32 / 1000.0
                        ),
                        false,
                    )
            })
        })
        .await;
    if let Err(why) = message {
        eprintln!("Error sending message: {:?}", why);
    };
}

async fn set_status_current_time(ctx: Arc<Context>) {
    let current_time = Utc::now();
    let guild_counts = ctx.cache.guild_count().await;

    let formatted = Activity::playing(format!(
        "Waiting for commands in {} guilds.\nCurrent Time: {}",
        guild_counts,
        current_time.to_rfc2822()
    ));

    ctx.set_activity(formatted).await;
}
