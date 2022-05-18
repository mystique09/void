use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::{db::users::get_user, BotDb};

#[group]
#[description = "A group of game commands."]
#[summary = "Game commands."]
#[prefix = "game"]
#[commands(spin, roll, guess, inventory, shop, give)]
pub struct GameCommands;

#[command]
#[description = "A spin game."]
async fn spin(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Spin game.").await?;
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
```\n?game guess {amount | default 4} {bet | default random}```"#]
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

    let amount = args.current().unwrap_or("").parse::<i64>().unwrap_or(8);
    args.advance();

    if amount < 0 {
        msg.channel_id
            .say(&ctx.http, "Amount should not be a negative.")
            .await?;
        return Ok(());
    }

    let randn_bet = {
        let mut rng = rand::thread_rng();
        let rand_n: u32 = rng.gen_range(1..6);
        rand_n
    };
    let bet = args
        .current()
        .unwrap_or("")
        .parse::<u32>()
        .unwrap_or(randn_bet);

    let user_data = get_user(&pool, &uid).await.unwrap();

    if amount > user_data.wallet || user_data.wallet < 1 {
        msg.channel_id.say(&ctx.http, "Not enough balance.").await?;
        return Ok(());
    }

    let rn = {
        let mut rng = rand::thread_rng();
        let random_num: u32 = rng.gen_range(1..6);
        random_num
    };

    if rn == bet {
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
                "Your guess is {}, guessed number is {}. You won ${}",
                bet, rn, amount
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
                "{}, our guess is {}, guessed number is {}. You lose ${}.",
                user_data.username, bet, rn, amount
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
#[description = "A spin game."]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Shop command.").await?;
    Ok(())
}

#[command]
#[description = "A command to give others coins."]
async fn give(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Give command.").await?;
    Ok(())
}
