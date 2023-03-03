use std::sync::Arc;

use serenity::{model::prelude::GuildId, prelude::Context};
use tracing::info;

use crate::domain::auto_respond::AutoRespondUsecase;

pub async fn bind_keywords(ctx: Arc<Context>, guilds: &[GuildId]) {
    let data = ctx.data.read().await;
    let usecase = data
        .get::<crate::bot::shared::SharedKeywordUsecase>()
        .unwrap()
        .clone();

    let keyword_usecase = usecase.write().await;
    let guild_state = data
        .get::<crate::bot::shared::SharedGuildState>()
        .unwrap()
        .clone();
    let mut guild_state = guild_state.write().await;

    for guild_id in guilds.iter() {
        let keywords = keyword_usecase
            .get_keywords(guild_id.0 as i64)
            .await
            .unwrap_or(vec![]);

        match guild_state.insert(
            *guild_id,
            crate::bot::shared::Guild {
                channels: vec![],
                bumps: vec![],
                keywords,
            },
        ) {
            Some(d) => info!(
                "keywords {:#?} for thks guild {} already exist, and is updated",
                d, guild_id,
            ),
            None => info!(
                "new guild detected, keywords is added for guild {}",
                &guild_id
            ),
        };
    }

    info!("keyword state lock: {:#?}", guild_state);
}
