pub mod general;
pub mod generator;

use std::collections::HashSet;

use serenity::all::{Context, Message, UserId};
use serenity::all::standard::macros::help;
use serenity::framework::standard::{
    Args,
    CommandGroup,
    CommandResult,
    help_commands,
    HelpOptions,
};

#[help]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
#[command_not_found_text = "Couldn't find: {}."]
pub async fn help_cmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await?;
    Ok(())
}