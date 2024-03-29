use std::sync::Arc;

use serenity::{
    model::prelude::command::Command,
    model::prelude::{
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOption},
        GuildId,
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
        "auto_respond" => auto_respond::set::run(Arc::clone(ctx), cmd, options).await,
        _ => "not implemented".to_string(),
    }
}

pub async fn register_local_commands(ctx: &Context, guild_id: &GuildId) {
    let commands = GuildId::set_application_commands(guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| bump::create_bump::register(command))
            .create_application_command(|command| bump::cancel_bump::register(command))
    })
    .await
    .map_err(|why| {
        error!(
            "cannot create local command: {}, for guild: {}",
            why, guild_id
        )
    })
    .unwrap();

    info!(
        "created {} slash commands for guild: {}",
        commands.len(),
        guild_id
    );
}

pub async fn register_global_commands(ctx: &Context) {
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

    Command::create_global_application_command(&ctx.http, |command| {
        auto_respond::set::register(command)
    })
    .await
    .map(|command| info!("created global command: {}", command.name))
    .map_err(|why| error!("error creating global command: {}", why))
    .unwrap();
}
