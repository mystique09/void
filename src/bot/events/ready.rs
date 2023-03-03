use std::sync::Arc;

use serenity::{model::prelude::GuildId, prelude::Context};
use tracing::info;

use crate::domain::auto_respond::AutoRespondUsecase;

pub async fn bind_keywords(ctx: Arc<Context>, guilds: &[GuildId]) {
    let data = ctx.data.read().await;
    let usecase = data
        .get::<crate::bot::shared::SharedUsecase>()
        .unwrap()
        .clone();

    let usecase = usecase.write().await;
    let guild_state = data
        .get::<crate::bot::shared::SharedGuildState>()
        .unwrap()
        .clone();
    let mut guild_state = guild_state.write().await;

    for guild_id in guilds.iter() {
        let keywords = usecase
            .keywords_usecase
            .get_keywords(guild_id.0 as i64)
            .await
            .unwrap_or(vec![]);

        match guild_state.get_mut(guild_id) {
            Some(guild) => {
                guild.keywords = keywords;
                info!(
                    "Guild [{}] Channels={} Bumps={} Keywords={}",
                    guild_id,
                    guild.channels.len(),
                    guild.bumps.len(),
                    guild.keywords.len()
                );
            }
            None => {
                let new_guild = crate::bot::shared::Guild {
                    channels: vec![],
                    bumps: vec![],
                    keywords,
                };
                guild_state.insert(*guild_id, new_guild).unwrap();
            }
        };
    }
}
