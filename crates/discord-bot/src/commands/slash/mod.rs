use std::sync::Arc;

use serenity::{
    model::prelude::command::Command,
    model::prelude::{
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOption},
        GuildId,
    },
    prelude::Context,
};

pub async fn run(
    ctx: &Arc<Context>,
    cmd: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> &'static str {
    match cmd.data.name.as_str() {
        "bump" => "bump",
        "autorespond" => "autorespond",
        _ => "not implemented",
    }
}

pub async fn register_local_commands(ctx: &Context, guild_id: &GuildId) {}
pub async fn register_global_commands(ctx: &Context) {}
