use crate::utils::random_number::randn;
use crate::{db::users::get_user, BotDb};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

const DEFAULT_BET: i64 = 20;

#[group]
#[description = "A group of game commands."]
#[summary = "Game commands."]
#[prefix = "game"]
#[commands(spin, roll, guess, inventory, shop, give)]
pub struct GameCommands;

#[command]
#[description = "A shop."]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Shop.").await?;
    Ok(())
}

#[command]
#[description = "A dice game."]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Dice game.").await?;
    Ok(())
}

#[command]
#[description = r#"A guessing game.
Usage: 
```
?game guess <amount> <guess>
0 < amount < balance
0 < guess < 6
```"#]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let uid = msg.author.id.to_string();

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

    let amount = args
        .current()
        .unwrap_or("")
        .parse::<i64>()
        .unwrap_or(DEFAULT_BET);
    args.advance();

    if amount < 0 {
        msg.channel_id
            .say(&ctx.http, "Amount should not be a negative.")
            .await?;
        return Ok(());
    }

    let randn_guess = randn(1..6).await;

    let guess = args
        .current()
        .unwrap_or("")
        .parse::<u32>()
        .unwrap_or(randn_guess);

    let user_data = get_user(&pool, &uid).await.unwrap();

    if amount > user_data.wallet || user_data.wallet < 1 {
        msg.channel_id.say(&ctx.http, "Not enough balance.").await?;
        return Ok(());
    }

    let rand_n = randn(1..6).await;

    if rand_n == guess {
        sqlx::query!(
            r#"
        UPDATE "profile"
        SET wallet = wallet + $1
        WHERE uid = $2
        "#,
            amount,
            uid
        )
        .execute(&pool)
        .await?
        .rows_affected();

        msg.reply(
            ctx,
            format!(
                "{}, your guess is {}, the correct number is {}. You won ${}.",
                user_data.username, &guess, &rand_n, amount
            ),
        )
        .await?;
    } else {
        sqlx::query!(
            r#"
        UPDATE "profile"
        SET wallet = CASE WHEN (wallet - $1) < 0
        THEN 0
        ELSE wallet - $1 END
        WHERE uid = $2
        "#,
            amount,
            uid
        )
        .execute(&pool)
        .await?
        .rows_affected();

        msg.reply(
            ctx,
            format!(
                "{}, your guess is {}, the correct number is {}. You lose ${}.",
                user_data.username, &guess, &rand_n, amount
            ),
        )
        .await?;
    }

    Ok(())
}

#[command]
#[description = "A command to show your inventory."]
async fn inventory(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Inventory command.").await?;
    Ok(())
}

#[command]
#[description = r#"A spin game.
Usage: 
```
?game spin <amount> <val>
0 < amount < balance
0 < val < 10
```"#]
async fn spin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // get the user's id
    let uid = msg.author.id.to_string();

    // get the pool
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

    // get the user's data
    let user_data = get_user(&pool, &uid).await.unwrap();

    // get the user's wallet
    let wallet = user_data.wallet;

    // get the user's username
    let username = user_data.username;

    // check if the user has available balance
    if wallet < 1 {
        msg.channel_id.say(&ctx.http, "Not enough balance.").await?;
        return Ok(());
    }

    // check if the user has enough arguments
    if args.len() < 2 {
        msg.channel_id
            .say(&ctx.http, "Not enough arguments.")
            .await?;
        return Ok(());
    }

    // validate if the argument is a number
    let amount = args.current().unwrap().parse::<i64>().unwrap_or(0);
    args.advance();
    let bet = args.current().unwrap().parse::<u32>().unwrap_or(0);

    // check if the amount is a positive number
    if amount < 1 {
        msg.channel_id
            .say(&ctx.http, "Amount should be a positive number.")
            .await?;
        return Ok(());
    }

    // check if the amount is less than the user's balance
    if amount > wallet {
        msg.channel_id.say(&ctx.http, "Not enough balance.").await?;
        return Ok(());
    }

    // check if the bet is a positive number
    if bet < 1 {
        msg.channel_id
            .say(&ctx.http, "Bet should be a positive number.")
            .await?;
        return Ok(());
    }

    // check if the bet is between 1-10
    if bet > 10 {
        msg.channel_id
            .say(&ctx.http, "Bet should be between 1-10.")
            .await?;
        return Ok(());
    }

    // generate random number between 1-10
    let rand_n = randn(1..10).await;

    // check if the user's guess is correct
    if rand_n == bet {
        // update the user's balance
        sqlx::query!(
            r#"
        UPDATE "profile"
        SET wallet = wallet + $1
        WHERE uid = $2
        "#,
            amount,
            uid
        )
        .execute(&pool)
        .await?
        .rows_affected();

        // send the message
        msg.reply(
            ctx,
            format!(
                "{}, your bet is {}, the result number is {}. You won ${}.",
                username, &bet, &rand_n, amount
            ),
        )
        .await?;
    } else {
        // update the user's balance
        sqlx::query!(
            r#"
        UPDATE "profile"
        SET wallet = CASE WHEN (wallet - $1) < 0
        THEN 0
        ELSE wallet - $1 END
        WHERE uid = $2
        "#,
            amount,
            uid
        )
        .execute(&pool)
        .await?
        .rows_affected();

        // send the message
        msg.reply(
            ctx,
            format!(
                "{}, your bet is {}, the result number is {}. You lose ${}.",
                username, &bet, &rand_n, amount
            ),
        )
        .await?;
    }

    Ok(())
}

#[command]
#[description = "A command to give others coins."]
async fn give(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Give command.").await?;
    Ok(())
}
