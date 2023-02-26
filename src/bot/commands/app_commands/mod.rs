use std::sync::Arc;

use serenity::{
    model::prelude::command::Command,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};
use tracing::{error, info};

pub mod admin;
pub mod auto_respond;
pub mod bump;
pub mod rss;

pub async fn match_app_command(
    ctx: &Arc<Context>,
    cmd: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    match cmd.data.name.as_str() {
        "bump" => bump::create_bump::run(Arc::clone(ctx), cmd, options).await,
        _ => "not implemented".to_string(),
    }
}

pub async fn register_commands(ctx: &Context) {
    match Command::create_global_application_command(&ctx.http, |command| {
        bump::create_bump::register(command)
    })
    .await
    {
        Ok(command) => info!("Created global app command: {}", command.name),
        Err(why) => error!("Error creating global command: {}", why),
    };
}
