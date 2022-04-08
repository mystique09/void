use serenity::framework::standard::macros::group;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[group]
#[owners_only]
#[description = "Only admins and mods can use this commands."]
#[summary = "Commands for admins and mods."]
#[prefixes("admin", "mods", "exc")]
#[commands(clean, ban, kick)]
pub struct AdminCommands;

#[command]
#[description = "Use to clean bulk of messages."]
pub async fn clean(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "[todo!] cleaned 100 messagges.").await?;
    Ok(())
}

#[command]
#[description = "Ban a user."]
pub async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "[todo!] Ban user command.").await?;
    Ok(())
}

#[command]
#[description = "Kick a user"]
pub async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "[todo!] Kick a user command.").await?;
    Ok(())
}
