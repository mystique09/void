use std::sync::Arc;

use crate::domain::auto_respond::KeywordUsecase;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::Permissions;
use serenity::{
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};
use tracing::{error, info};

use crate::bot::shared::{SharedKeywordUsecase, SharedKeywordsState};

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

    let shared_keyword_usecase = {
        let kw_lock = data.get::<SharedKeywordUsecase>().unwrap().clone();
        kw_lock
    };
    let keyword_usecase = shared_keyword_usecase.read().await;

    let shared_keyword_state = {
        let kw_state_lock = data.get::<SharedKeywordsState>().unwrap().clone();
        kw_state_lock
    };
    let mut keyword_state = shared_keyword_state.write().await;

    match keyword_usecase.create_keyword(new_keyword).await {
        Ok(v) => {
            match keyword_state.get_mut(&guild_id) {
                Some(kws) => {
                    kws.push(v);
                    info!("new keyword added: {:#?}", kws);
                }
                None => {
                    let keywords: Vec<crate::domain::auto_respond::Keyword> = vec![v];
                    info!(
                        "shared state is empty, attempt to create new: {:#?}",
                        &keywords
                    );
                    keyword_state.insert(guild_id, keywords);
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

pub fn register(commands: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    commands
        .name("auto_respond")
        .description("create a new auto respond")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .create_option(|option| {
            option
                .name("keyword")
                .description("the word to detect")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("response")
                .description("the response of the detected word")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("type")
                .description("what kind of response should be sent")
                .kind(CommandOptionType::String)
                .add_string_choice("Single line", "SINGLE")
                .add_string_choice("Multi line", "MULTI")
                .add_string_choice("Media", "MEDIA")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("mode")
                .description("where the response should be sent")
                .kind(CommandOptionType::String)
                .add_string_choice("Direct Message", "DM")
                .add_string_choice("Regular Message", "REGULAR")
                .required(true)
        })
}
