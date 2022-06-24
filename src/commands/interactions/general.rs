use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::{Message},
    utils::{Color, Colour},
};

use crate::{db::users::get_user, BotDb};

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

    let user = get_user(&pool, &msg.author.id.to_string()).await.unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s rank", msg.author.name))
                    .color(Color::PURPLE)
                    .description(format!(":crown: Rank {}", user.rank))
                    .timestamp(chrono::Utc::now())
            })
        })
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

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{} avatar", msg.author.name))
                    .image(avatar_url)
                    .timestamp(chrono::Utc::now())
            })
        })
        .await?;

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
    SELECT rank, username
    FROM "profile"
    ORDER BY rank
    DESC
    LIMIT 10
    "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let embed = e
                    .title(":star: Leaderboard :star:")
                    .description("Below is the list of top 10 users sorted by their rank")
                    .colour(Colour::BLUE)
                    .timestamp(chrono::Utc::now());

                users.into_iter().for_each(|user| {
                    embed.field(format!(":crown: Rank {}", user.rank), user.username, true);
                });

                embed
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description = "Get the server's latency."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let user_tmstmp = msg.timestamp;
    let now = Utc::now();
    let ping = (now - user_tmstmp).num_milliseconds();

    msg.reply_ping(&ctx.http, format!("```The ping is {} ms.```", ping))
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

    let user = get_user(&pool, &msg.author.id.to_string()).await.unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s balance", msg.author.name))
                    .color(Color::PURPLE)
                    .description(format!(
                        ":moneybag: **Wallet** ${} \n:bank: **Bank** ${} \n:small_blue_diamond: **Diamonds** ${}",
                        user.wallet, user.bank, user.diamond
                    ))
                    .timestamp(chrono::Utc::now())
            })
        })
        .await?;

    Ok(())
}
