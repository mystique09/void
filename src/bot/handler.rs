use chrono::Utc;
use serenity::{
    async_trait,
    model::{
        prelude::{
            command::Command,
            interaction::{Interaction, InteractionResponseType},
            Activity, ChannelId, GuildId, Message, Ready,
        },
        user::OnlineStatus,
    },
    prelude::{Context, EventHandler},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tracing::{error, info};

use super::shared::{SharedGuildState, SharedEnvState};

pub struct BotHandler {
    pub is_parallelized: AtomicBool,
}

// cooldown for sending system resource
// let's set this to 2 minutes
const COOLDOWN: u64 = 60;

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity: Option<Activity> = Some(Activity::playing("NeoVim"));

        ctx.set_presence(activity, OnlineStatus::Online).await;
        info!("{} is now open.", &ready.user.name);

        match Command::create_global_application_command(&ctx.http, |command| {
            super::commands::app_commands::bump::create_bump::register(command)
        })
        .await
        {
            Ok(command) => error!("Created global app command: {}", command.name),
            Err(why) => error!("Error creating global command: {}", why),
        };
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let ctxcpy = Arc::new(ctx);

            let content = match command.data.name.as_str() {
                "bump" => {
                    super::commands::app_commands::bump::create_bump::run(
                        Arc::clone(&ctxcpy),
                        &command.data.options,
                    )
                    .await
                }
                _ => "not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctxcpy, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        };

        let _data = ctx
            .data
            .read()
            .await
            .get::<SharedGuildState>()
            .unwrap()
            .clone();
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Cache built successfuly.");

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

        let ctxcpy3 = Arc::clone(&ctx);

        tokio::spawn(async move {
            let guilds_cache = {
                let data = ctxcpy3.data.read().await;
                data.get::<SharedGuildState>().unwrap().clone()
            };

            for guild_id in guilds.iter() {
                let mut guild_cache_lock = guilds_cache.write().await;
                let guild = ctxcpy3.cache.guild(guild_id).unwrap();
                let channels = guild
                    .channels
                    .into_iter()
                    .map(|c| (c.1.to_string(), c.0))
                    .collect();

                guild_cache_lock.insert(guild.id, crate::bot::shared::Guild { channels });
            }
        });
    }
}

async fn log_system_load(ctx: Arc<Context>) {
    let data = ctx
    .data
    .read()
    .await
    .get::<SharedEnvState>()
    .unwrap()
    .clone();

    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();
    let data_lock = data.write().await;
    let channel_id = data_lock.get_channel_id();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(*channel_id)
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
        error!("Error sending message: {:?}", why);
    };
}

async fn set_status_current_time(ctx: Arc<Context>) {
    let current_time = Utc::now();
    let guild_counts = ctx.cache.guild_count();

    let formatted = Activity::playing(format!(
        "Waiting for commands in {} guilds.\nCurrent Time: {}",
        guild_counts,
        current_time.to_rfc2822()
    ));

    ctx.set_activity(formatted).await;
}
