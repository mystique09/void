use serenity::framework::standard::macros::group;
use serenity::framework::standard::Args;
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
pub async fn clean(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // get the amount of messages to delete ffrom argument , default is 11
    let amount = args.single::<u64>().unwrap_or(11);

    // prune the messages  based on the amount
    let _ = msg
        .channel_id
        .messages(&ctx.http, |m| m.limit(amount))
        .await;

    Ok(())
}

#[command]
#[description = "Ban a user."]
pub async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // get the mentioned user in the argument
    let user = match args.single::<u64>() {
        Ok(user) => user,
        Err(_) => {
            msg.reply(ctx, "Please mention a user to ban.").await?;
            return Ok(());
        }
    };

    // ban the user
    let _ = msg.guild_id.unwrap().ban(&ctx.http, user, 4).await;
    // Notify the server owner that a user has been banned
    // mention the user that has been banned
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("{} has been banned.", user))
        .await;

    Ok(())
}

#[command]
#[description = "Kick a user"]
pub async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // get the mentioned user in the argument
    let user = match args.single::<u64>() {
        Ok(user) => user,
        Err(_) => {
            msg.reply(ctx, "Please mention a user to kick.").await?;
            return Ok(());
        }
    };

    // kick the user
    let _ = msg.guild_id.unwrap().kick(&ctx.http, user).await;
    // Notify the server owner that a user has been kicked
    // mention the user that has been kicked
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("{} has been kicked.", user))
        .await;

    Ok(())
}
