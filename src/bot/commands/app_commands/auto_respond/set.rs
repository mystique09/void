use std::sync::Arc;

use crate::domain::auto_respond::AutoRespondUsecase;
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::ChannelId;
use serenity::{
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};
use tracing::{error, info};

use crate::bot::shared::{Guild, SharedGuildState, SharedUsecase};

pub async fn run(
    ctx: Arc<Context>,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let new_keyword = {
        let keyword = {
            let kw = options
                .get(0)
                .expect("keyword")
                .resolved
                .as_ref()
                .expect("keyword");

            if let CommandDataOptionValue::String(v) = kw {
                v
            } else {
                return "missing keyword".to_string();
            }
        };

        let response = {
            let res = options
                .get(1)
                .expect("response")
                .resolved
                .as_ref()
                .expect("response");

            if let CommandDataOptionValue::String(v) = res {
                v
            } else {
                return "missing response".to_string();
            }
        };

        let response_type = {
            let res_type = options
                .get(2)
                .expect("response type")
                .resolved
                .as_ref()
                .expect("response type");

            if let CommandDataOptionValue::String(v) = res_type {
                v
            } else {
                return "missing response type".to_string();
            }
        };

        let response_mode = {
            let res_mode = options
                .get(3)
                .expect("response mode")
                .resolved
                .as_ref()
                .expect("response mode");

            if let CommandDataOptionValue::String(v) = res_mode {
                v
            } else {
                return "missing response type".to_string();
            }
        };

        crate::domain::auto_respond::CreateKeywordDTO {
            id: uuid::Uuid::new_v4(),
            word: keyword.into(),
            guild_id: command.guild_id.unwrap().0 as i64,
            response: response.into(),
            response_type: crate::domain::auto_respond::ResponseType::from(response_type.as_str()),
            response_mode: crate::domain::auto_respond::ResponseMode::from(response_mode.as_str()),
        }
    };

    let guild_id = command.guild_id.unwrap();
    let data = ctx.data.read().await;

    let shared_usecase = {
        let kw_lock = data.get::<SharedUsecase>().unwrap().clone();
        kw_lock
    };
    let usecase = shared_usecase.read().await;

    let guild_state = {
        let kw_state_lock = data.get::<SharedGuildState>().unwrap().clone();
        kw_state_lock
    };
    let mut guild = guild_state.write().await;

    match usecase.keywords_usecase.create_keyword(new_keyword).await {
        Ok(v) => {
            match guild.get_mut(&guild_id) {
                Some(kws) => {
                    info!("new keyword added: {:#?}", &v);
                    kws.keywords.push(v);
                }
                None => {
                    let keywords: Vec<crate::domain::auto_respond::Keyword> = vec![v];
                    info!(
                        "shared state is empty, attempt to create new: {:#?}",
                        &keywords
                    );

                    let guild_cache = ctx.cache.guild(guild_id).unwrap();
                    let channels: Vec<(String, ChannelId)> = guild_cache
                        .channels
                        .into_iter()
                        .map(|c| (c.1.to_string(), c.0))
                        .collect();

                    guild.insert(
                        guild_id,
                        Guild {
                            bumps: vec![],
                            keywords,
                            channels,
                        },
                    );
                }
            };

            "new keyword added".to_string()
        }
        Err(why) => {
            error!("{}", why);
            "something went wrong".to_string()
        }
    }
}

pub fn register(
    commands: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
    commands
        .name("add")
        .description("create a new auto respond")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|option| {
            option
                .name("keyword")
                .description("the word to detect")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|option| {
            option
                .name("response")
                .description("the response of the detected word")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_sub_option(|option| {
            option
                .name("type")
                .description("what kind of response should be sent")
                .kind(CommandOptionType::String)
                .add_string_choice("Single line", "SINGLE")
                .add_string_choice("Multi line", "MULTI")
                .add_string_choice("Media", "MEDIA")
                .required(true)
        })
        .create_sub_option(|option| {
            option
                .name("mode")
                .description("where the response should be sent")
                .kind(CommandOptionType::String)
                .add_string_choice("Direct Message", "DM")
                .add_string_choice("Regular Message", "REGULAR")
                .required(true)
        })
}
