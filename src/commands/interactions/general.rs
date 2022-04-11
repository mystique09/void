use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    utils::Colour,
};

use crate::{
    db::users::{get_user, TUser},
    BotDb,
};

#[group]
#[description = "Group of general commands."]
#[summary = "General commands."]
#[commands(ping, avatar, rank, leaderboard, balance)]
pub struct GeneralCommands;

#[command]
#[description = "Fetch the user's rank."]
async fn rank(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<BotDb>()
        .unwrap()
        .clone()
        .read()
        .unwrap()
        .clone();

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
        .unwrap_or_else(|| msg.author.default_avatar_url());

    msg.reply_ping(ctx, avatar_url).await?;
    Ok(())
}

#[command]
#[description = "Get the server's leaderboard."]
#[aliases("lb")]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<BotDb>()
        .unwrap()
        .clone()
        .read()
        .unwrap()
        .clone();

    let users = sqlx::query!(
        r#"
    SELECT user_rank, dc_id
    FROM "user"
    WHERE user_id < 11
    ORDER BY user_rank
    DESC
    "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let field_embed = users
        .into_iter()
        .map(|user| {
            (
                format!("<@&{}>", user.dc_id),
                format!("Rank {}", user.user_rank),
                true,
            )
        })
        .collect::<Vec<(String, String, bool)>>()
        .into_iter();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content("Leaderboard").embed(|e| {
                e.title(":star: Leaderboard :star:")
                    .description("Below is the list of top 10 users sorted by their rank")
                    .colour(Colour::BLUE)
                    .fields(field_embed)
                    .timestamp(chrono::Utc::now())
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description = "Get the server's latency."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
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
    let pool = ctx
        .data
        .read()
        .await
        .get::<BotDb>()
        .unwrap()
        .clone()
        .read()
        .unwrap()
        .clone();

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
