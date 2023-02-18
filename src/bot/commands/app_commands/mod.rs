use std::sync::Arc;

use serenity::{
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};

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
