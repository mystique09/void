use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[description = "A group of fun commands."]
#[summary = "Fun commands."]
#[prefix = "fun"]
#[commands(meme, nuke, art, pwdgen)]
pub struct FunCommands;

#[command]
#[description = "Fetch random meme from internet."]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Random meme command.").await?;
    Ok(())
}

#[command]
#[description = "Nuke someone with this command."]
async fn nuke(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Nuke someone.").await?;
    Ok(())
}

#[command]
#[description = "Fetch random art from internet."]
async fn art(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Random art command.").await?;
    Ok(())
}

#[command]
#[description = "Generate password."]
async fn pwdgen(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Password generator command.").await?;
    Ok(())
}
