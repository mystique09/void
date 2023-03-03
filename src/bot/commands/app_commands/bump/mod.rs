use serenity::builder::CreateApplicationCommand;

pub mod cancel_bump;
pub mod create_bump;

use std::sync::Arc;

use serenity::{
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};

pub async fn run(
    ctx: Arc<Context>,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let field_name = options.first().unwrap();

    match field_name.name.as_str() {
        "create" => create_bump::run(Arc::clone(&ctx), command, &field_name.options).await,
        "cancel" => cancel_bump::run(Arc::clone(&ctx), command, &field_name.options).await,
        _ => "command not found".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bump")
        .description("bump command")
        .create_option(create_bump::register)
        .create_option(cancel_bump::register)
}
