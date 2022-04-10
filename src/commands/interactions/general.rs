use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

use crate::{
    db::users::{get_user, TUser},
    BotDb,
};

#[group]
#[description = "Group of general commands."]
#[summary = "General commands."]
#[commands(latency, ping, avatar, rank, leaderboard, balance)]
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
    let db = {
        let db_read = ctx.data.read().await;
        db_read.get::<BotDb>().unwrap().clone()
    };

    let pool = db.read().unwrap().clone();

    let user = get_user(&pool, *msg.author.id.as_u64() as i64)
        .await
        .unwrap();

    msg.reply_ping(
        ctx,
        format!(
            "```js\nYour current rank is {}, Exp: ( {}/20 )```",
            user.get_rank(),
            user.get_exp()
        ),
    )
    .await?;
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
#[description = "Get the server's latency."]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let user_ping = msg.timestamp;
    let l_msg = msg
        .channel_id
        .say(ctx, "Caculating latency ...")
        .await
        .unwrap();
    l_msg.delete(ctx).await?;

    let latency = (l_msg.timestamp - user_ping).num_milliseconds();

    msg.channel_id
        .say(ctx, format!("```js\nLatency is {} ms.```", latency))
        .await?;

    Ok(())
}

#[command]
#[description = "Get the user's balance."]
#[aliases("bal")]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let db = {
        let db_read = ctx.data.read().await;
        db_read.get::<BotDb>().unwrap().clone()
    };

    let pool = db.read().unwrap().clone();

    let user = get_user(&pool, *msg.author.id.as_u64() as i64)
        .await
        .unwrap();

    msg.reply_ping(
        ctx,
        format!("```js\nYour current balance is ${}.```", user.get_balance()),
    )
    .await?;
    Ok(())
}
