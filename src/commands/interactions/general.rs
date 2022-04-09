use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[description = "Group of general commands."]
#[summary = "General commands."]
#[commands(ping, avatar, rank, leaderboard, balance)]
pub struct GeneralCommands;

#[command]
#[description = "Ping command."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "Pong!").await?;
    Ok(())
}

#[command]
#[description = "Fetch the user's rank."]
async fn rank(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "Rank #1").await?;
    Ok(())
}

#[command]
#[description = "Fetch the user's avatar command."]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let avatar_url = msg
        .author
        .avatar_url()
        .unwrap_or(msg.author.default_avatar_url());

    msg.reply_ping(ctx, avatar_url).await?;
    Ok(())
}

#[command]
#[description = "Get the server's leaderboard."]
#[aliases("lb")]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "Leaderboard.").await?;
    Ok(())
}

#[command]
#[description = "Get the user's balance."]
#[aliases("bal")]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "Balance.").await?;
    Ok(())
}
