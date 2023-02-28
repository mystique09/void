use std::sync::Arc;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::{
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};
//use tracing::{error, info};

pub async fn run(
    _ctx: Arc<Context>,
    _command: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> String {
    //todo!("implement set method");
    "new keyword added".to_string()
}

pub fn register(commands: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    commands
        .name("auto_respond")
        .description("create a new auto respond")
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
