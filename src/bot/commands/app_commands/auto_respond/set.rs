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

    let data = ctx.data.read().await;
    let keyword_usecase_lock = {
        let kw_lock = data.get::<SharedKeywordUsecase>().unwrap().clone();
        kw_lock
    };
    let keyword_usecase = keyword_usecase_lock.read().await;
    let keyword_state_lock = {
        let kw_state_lock = data.get::<SharedKeywordsState>().unwrap().clone();
        kw_state_lock
    };
    let mut keyword_state = keyword_state_lock.write().await;

    match keyword_usecase.create_keyword(new_keyword).await {
        Ok(v) => {
            info!("new keyword created {}", &v.word);
            let guild = keyword_state.get_mut(&command.guild_id.unwrap()).unwrap();
            guild.push(v);

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
