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
        "bump_cancel" => bump::cancel_bump::run(Arc::clone(ctx), cmd, options).await,
        _ => "not implemented".to_string(),
    }
}

pub async fn register_commands(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        bump::create_bump::register(command)
    })
    .await
    .map(|command| info!("created global command: {}", command.name))
    .map_err(|why| error!("error creating global command: {}", why))
    .unwrap();

    Command::create_global_application_command(&ctx.http, |command| {
        bump::cancel_bump::register(command)
    })
    .await
    .map(|command| info!("created global command: {}", command.name))
    .map_err(|why| error!("error creating global command: {}", why))
    .unwrap();
}
