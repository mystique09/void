use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::Mentionable,
};

use crate::utils::random_number::randn;

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
    // get the mentioned user and current user
    let user = msg.mentions.first().unwrap();
    let author = msg.author.clone();

    // send the message
    msg.channel_id
        .say(
            ctx,
            format!("{} nuked {}!", author.mention(), user.mention()),
        )
        .await?;

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
async fn pwdgen(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // strings, symbols, and numbers
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+";
    let len = chars.len() as u32;
    let mut pwd = String::new();

    // get the password length
    let length = args.single::<u8>().unwrap_or(8);

    // generate the password
    for _ in 0..length {
        let n = randn(1..len).await as usize;
        pwd.push(chars.chars().nth(n).unwrap());
    }

    // send the message in an embed format with a button to allow user to copy the generated password
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Password")
                    .description(pwd.to_string())
                    .color(0x00ff00)
                    .footer(|f| f.text("Generated by VoidBot"))
                    .author(|a| a.name("Password Generator"))
            })
        })
        .await?;

    Ok(())
}