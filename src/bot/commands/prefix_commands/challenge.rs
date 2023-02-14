use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[description = "A group of challenge commands."]
#[summary = "This is a group of challange commands."]
#[prefixes("challenge", "chall")]
#[commands(draw, code)]
pub struct ChallengeCommands;

#[command]
#[description = "Drawing challenge."]
async fn draw(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Drawing challenge command.").await?;
    Ok(())
}

#[command]
#[description = "Coding challenge."]
async fn code(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Coding challenge command.").await?;
    Ok(())
}
