mod commands;

use std::collections::HashSet;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::CommandGroup;
use serenity::framework::standard::{
    help_commands,
    macros::{command, group},
    Args, CommandResult, HelpOptions, StandardFramework,
};
use serenity::http::Http;
use serenity::model::id::UserId;
use serenity::model::{channel::Message, gateway::Ready};

use crate::admin::ADMINCOMMANDS_GROUP;
use crate::challenge::CHALLENGECOMMANDS_GROUP;
use crate::fun::FUNCOMMANDS_GROUP;
use crate::game::GAMECOMMANDS_GROUP;

use crate::commands::interactions::{admin, challenge, fun, game};

#[group]
#[description = "A group of all general commands."]
#[summary = "General commands, bitch!"]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is now open.", &ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let http = Http::new_with_token(&std::env::var("TOKEN").unwrap());

    let (owner_ids, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id),
                Err(why) => panic!("Error:  {}", why),
            }
        }
        Err(why) => panic!("Error: {}", why),
    };

    let fm = StandardFramework::new()
        .configure(|c| c.prefix("?").with_whitespace(true).owners(owner_ids))
        .group(&GENERAL_GROUP)
        .group(&ADMINCOMMANDS_GROUP)
        .group(&FUNCOMMANDS_GROUP)
        .group(&GAMECOMMANDS_GROUP)
        .group(&CHALLENGECOMMANDS_GROUP)
        .help(&HELP);

    let token = std::env::var("TOKEN").unwrap();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(fm)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        eprintln!("An error occured: {}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_mention(ctx, "Pong!").await?;
    Ok(())
}

#[help]
#[command_not_found_text = "Could not execute {} command, command doesn't exist."]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
