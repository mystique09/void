use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
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

    let user = get_user(&pool, *msg.author.id.as_u64() as i64)
        .await
        .unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s rank", msg.author.name))
                    .color(Color::PURPLE)
                    .description(format!("Rank :crown: {}", user.user_rank))
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
    SELECT user_rank, user_name
    FROM "user"
    ORDER BY user_rank
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
                    embed.field(
                        format!(":crown: Rank {}", user.user_rank),
                        user.user_name,
                        true,
                    );
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
    let user_ping = msg.timestamp;
    let l_msg = msg
        .channel_id
        .say(ctx, "Caculating latency ...")
        .await
        .unwrap();
    l_msg.delete(ctx).await?;

    let latency = (l_msg.timestamp - user_ping).num_milliseconds();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(":rocket: Server Latency :rocket:")
                    .color(Color::DARK_GREEN)
                    .description(format!("Latency is {} ms.", latency))
                    .timestamp(chrono::Utc::now())
            })
        })
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

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s balance", msg.author.name))
                    .color(Color::PURPLE)
                    .description(format!(":moneybag: ${}", user.user_balance))
                    .timestamp(chrono::Utc::now())
            })
        })
        .await?;

    Ok(())
}
