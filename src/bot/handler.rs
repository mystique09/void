use crate::bot::commands::app_commands::register_global_commands;
use crate::domain::auto_respond::KeywordUsecase;
use chrono::Utc;
use serenity::{
    async_trait,
    model::{
        prelude::{
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

use super::shared::{SharedEnvState, SharedGuildState};

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
        info!("{} is now open.", &ready.user.name);

        register_global_commands(&ctx).await;

        /*
        fetch all keywords from db and save in shared cache
        */
        let data = ctx.data.read().await;
        let usecase = data
            .get::<super::shared::SharedKeywordUsecase>()
            .unwrap()
            .clone();
        let guilds = ctx.cache.guilds();

        let auto_respond_usecase_lock = usecase.write().await;
        let keyword_state = data
            .get::<super::shared::SharedKeywordsState>()
            .unwrap()
            .clone();
        let mut keyword_state_lock = keyword_state.write().await;

        for guild_id in guilds.iter() {
            let keywords = auto_respond_usecase_lock
                .get_keywords(guild_id.0 as i64)
                .await
                .unwrap_or(vec![]);

            match keyword_state_lock.insert(*guild_id, keywords) {
                Some(d) => info!(
                    "keywords {:#?} for thks guild {} already exist, and is updated",
                    d, guild_id,
                ),
                None => info!("new guild, keywords is added for guild {}", &guild_id),
            };
        }

        info!("{:#?}", keyword_state_lock);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let ctxcpy = Arc::new(ctx);

            let content = super::commands::app_commands::match_app_command(
                &ctxcpy,
                &command,
                &command.data.options,
            )
            .await;

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

        /*
        implement auto respond feature
        In ready event all the keywords must be fetch and saved in shared cache
        to avoid many calls in the db, only fetch again the db when new
        keyword is added/updated/deleted.
        */
        let data = ctx
            .data
            .read()
            .await
            .get::<super::shared::SharedKeywordsState>()
            .unwrap()
            .clone();

        let keywords_cache = data.read().await;
        let keywords = keywords_cache.get(&message.guild_id.unwrap()).unwrap();

        for kw in keywords.iter() {
            if message.content.contains(&kw.word) {
                message
                    .channel_id
                    .send_message(&ctx.http, |m| m.content(&kw.response))
                    .await
                    .unwrap();
            }
        }
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Cache built successfuly.");

        /*
        We register the slash commands for each guild here instead of the ready event,
        since the guild cache is not yet ready when the bot is ready(well, obviously).

        I don't have any use for this so let's comment it.

        POTENTIAL-USE: custom commands for a guild(?)
        */
        /*
        for guild_id in guilds.iter() {
            register_local_commands(&ctx, guild_id).await;
        }
        */

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

        /*
        Set the guilds in cache for use later.
        */
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

    let cpu_load = match sys_info::loadavg() {
        Ok(cpu) => format!("{:.2}%", cpu.one * 10.0),
        Err(why) => {
            error!("{}", why);
            "not available".to_string()
        }
    };
    let mem_use = match sys_info::mem_info() {
        Ok(mem) => format!(
            "{:.2} MB Free out of {:.2}",
            mem.free as f32 / 1000.0,
            mem.total as f32 / 1000.0
        ),
        Err(why) => {
            error!("{}", why);
            "not available".to_string()
        }
    };
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

    let formatted = Activity::playing(format!(
        "Waiting for commands in {} guilds.\nCurrent Time: {}",
        guild_counts,
        current_time.to_rfc2822()
    ));

    ctx.set_activity(formatted).await;
}
