use std::sync::Arc;

use serenity::{
    model::prelude::{GuildId, Message},
    prelude::Context,
};

use crate::domain::auto_respond::{ResponseMode, ResponseType};

/*
implement auto respond feature
In ready event all the keywords must be fetch and saved in shared cache
to avoid many calls in the db, only fetch again the db when new
keyword is added/updated/deleted.
*/
pub async fn auto_respond_event(ctx: Arc<Context>, message: &Message, guild_id: &GuildId) {
    let data = ctx
        .data
        .read()
        .await
        .get::<crate::bot::shared::SharedGuildState>()
        .unwrap()
        .clone();

    let mut guild_state = data.write().await;
    let guild = match guild_state.get(guild_id) {
        Some(cache) => cache,
        None => {
            let keywords: Vec<crate::domain::auto_respond::Keyword> = vec![];
            guild_state.insert(
                *guild_id,
                crate::bot::shared::Guild {
                    channels: vec![],
                    bumps: vec![],
                    keywords,
                },
            );
            let cache = guild_state.get_mut(guild_id).unwrap();
            cache
        }
    };

    for kw in guild.keywords.iter() {
        // TODO!: match the response type and mode to be sent
        if message.content.contains(&kw.word) {
            match kw.response_type {
                ResponseType::SingleLine => {
                    match kw.response_mode {
                        ResponseMode::Regular => {
                            message
                                .channel_id
                                .send_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                        ResponseMode::DirectMessage => {
                            message
                                .author
                                .direct_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                    };
                }
                ResponseType::MultiLine => {
                    // TODO!: needs to find a way to differentiate the response
                    match kw.response_mode {
                        ResponseMode::Regular => {
                            message
                                .channel_id
                                .send_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                        ResponseMode::DirectMessage => {
                            message
                                .author
                                .direct_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                    };
                }
                ResponseType::Media => {
                    match kw.response_mode {
                        ResponseMode::Regular => {
                            message
                                .channel_id
                                .send_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                        ResponseMode::DirectMessage => {
                            message
                                .author
                                .direct_message(&ctx.http, |m| m.content(&kw.response))
                                .await
                                .unwrap();
                        }
                    };
                }
            };
            break;
        }
    }
}
