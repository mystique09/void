pub mod delete;
pub mod edit;
pub mod set;

use serenity::{builder::CreateApplicationCommand, model::Permissions};

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
        "add" => set::run(Arc::clone(&ctx), command, &field_name.options).await,
        _ => "command not found".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("auto_respond")
        .description("bump command")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .create_option(set::register)
}
