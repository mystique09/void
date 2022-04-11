use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::BotDb;

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
#[description = "A guessing game."]
async fn guess(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_id = *msg.author.id.as_u64() as i64;

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

    let parse_arg = match args.parse::<u32>() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if parse_arg == 0 {
        msg.channel_id
            .say(ctx, "Are you dumb? I want a number, not a string!")
            .await?;
        return Ok(());
    }

    let rn = {
        let mut rng = rand::thread_rng();
        let random_num: u32 = rng.gen_range(1..5);
        random_num
    };

    if rn == parse_arg {
        sqlx::query!(
            r#"
        UPDATE "user"
        SET user_balance = user_balance + 10
        WHERE dc_id = $1
        "#,
            user_id
        )
        .execute(&pool)
        .await?
        .rows_affected();

        msg.reply(
            ctx,
            format!(
                "Your guess is {}, guessed number is {}. You won $10",
                parse_arg, rn
            ),
        )
        .await?;
    } else {
        sqlx::query!(
            r#"
        UPDATE "user"
        SET user_balance = user_balance - 2
        WHERE dc_id = $1
        AND user_balance > 0
        "#,
            user_id
        )
        .execute(&pool)
        .await?
        .rows_affected();

        msg.reply(
            ctx,
            format!(
                "Your guess is {}, guessed number is {}. You lose.",
                parse_arg, rn
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
