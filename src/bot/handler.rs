use crate::bot::commands::app_commands::register_global_commands;
use crate::bot::events::cache::{bind_guilds, log_system, set_bot_status};
use crate::bot::events::ready::bind_keywords;
use serenity::{
    async_trait,
    model::{
        prelude::{
            interaction::{Interaction, InteractionResponseType},
            Activity, GuildId, Message, Ready,
        },
        user::OnlineStatus,
    },
    prelude::{Context, EventHandler},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tracing::{error, info};

use super::events::message::auto_respond_event;

pub struct BotHandler {
    pub is_parallelized: AtomicBool,
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity: Option<Activity> = Some(Activity::playing("NeoVim"));
        ctx.set_presence(activity, OnlineStatus::Online).await;
        info!("{} is now open.", &ready.user.name);

        register_global_commands(&ctx).await;

        let guilds = ctx.cache.guilds();
        let ctxcpy = Arc::new(ctx);
        bind_keywords(Arc::clone(&ctxcpy), &guilds).await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Cache built successfuly.");

        /*
        We register the local slash commands for each guild here instead of the ready event,
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
            log_system(ctxcpy1).await;

            let ctxcpy2 = Arc::clone(&ctx);
            set_bot_status(ctxcpy2).await;

            self.is_parallelized.swap(true, Ordering::Relaxed);
        }

        let ctxcpy3 = Arc::clone(&ctx);

        /*
        Set the guilds in cache for use later.
        */
        bind_guilds(ctxcpy3, guilds).await;
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

        let guild_id = message.guild_id.unwrap();

        let ctxcpy = Arc::new(ctx);
        auto_respond_event(Arc::clone(&ctxcpy), &message, &guild_id).await;
    }
}
