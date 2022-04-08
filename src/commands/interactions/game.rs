use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

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
async fn guess(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Guessing game.").await?;
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
