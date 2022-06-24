mod commands;
mod db;
mod utils;

use crate::commands::interactions::{admin, challenge, fun, game, general};
use crate::db::users::{delete_user, get_user, new_user, set_exp};
use admin::ADMINCOMMANDS_GROUP;
use challenge::CHALLENGECOMMANDS_GROUP;
use fun::FUNCOMMANDS_GROUP;
use game::GAMECOMMANDS_GROUP;
use general::GENERALCOMMANDS_GROUP;
use serenity::model::guild::Member;
use serenity::model::prelude::{Activity, OnlineStatus};

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::CommandGroup;
use serenity::framework::standard::{
    help_commands, Args, CommandResult, HelpOptions, StandardFramework,
};
use serenity::http::Http;
use serenity::model::id::{GuildId, UserId};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPool;

struct BotDb;

impl TypeMapKey for BotDb {
    type Value = Arc<RwLock<PgPool>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity: Option<Activity> = Some(Activity::playing("NeoVim"));

        ctx.set_presence(activity, OnlineStatus::Online).await;
        println!("{} is now open.", &ready.user.name);
    }

    async fn message(&self, _ctx: Context, _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        let pool = _ctx
            .data
            .read()
            .await
            .get::<BotDb>()
            .unwrap()
            .clone()
            .read()
            .unwrap()
            .clone();

        let user_id = _new_message.author.id.to_string();
        let guild_id = _new_message.guild_id.unwrap().to_string();

        let check_user = db::users::get_user(&pool, &user_id).await;

        match check_user {
            Ok(user) => {
                if user.exp == 19 {
                    _new_message
                        .reply_mention(
                            _ctx,
                            format!("Congrats you level up! Rank {}", user.rank + 1),
                        )
                        .await
                        .unwrap();
                }

                set_exp(&pool, &user).await.unwrap();
            }
            Err(sqlx::Error::RowNotFound) => {
                if _new_message.author.bot {
                    return;
                }

                let uid = new_user(&pool, &user_id, &guild_id, &_new_message.author.name)
                    .await
                    .unwrap();
                let new_user = get_user(&pool, &uid).await.unwrap();

                println!("New user initialized: {}", new_user.uid);
            }
            Err(why) => println!("ERRORR: {:?}", why),
        }
    }

    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
        if !_new_member.pending {
            let uid = _new_member.user.id.to_string();
            let gid = _new_member.guild_id.to_string();
            let name = _new_member.user.name;

            println!("{} joined the server. ID: {}", name, gid);

            let pool = _ctx
                .data
                .read()
                .await
                .get::<BotDb>()
                .unwrap()
                .clone()
                .read()
                .unwrap()
                .clone();

            new_user(&pool, &uid, &gid, &name).await.unwrap();
        }
    }

    async fn guild_member_removal(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _user: serenity::model::prelude::User,
        _member_data_if_available: Option<Member>,
    ) {
        let (user_name, user_id) = match _member_data_if_available {
            Some(data) => (data.user.name, data.user.id),
            None => ("unknown".to_string(), UserId::from(0)),
        };

        let pool = _ctx
            .data
            .read()
            .await
            .get::<BotDb>()
            .unwrap()
            .clone()
            .read()
            .unwrap()
            .clone();

        delete_user(&pool, &user_id.to_string()).await.unwrap();

        println!("{} leave the server, ID: {}", user_name, user_id);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = dotenv::var("TOKEN").unwrap();
    let db_config = dotenv::var("DATABASE_URL").unwrap();

    let http = Http::new_with_token(&token);

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

    let db = PgPool::connect(&db_config).await.unwrap();

    let fm = StandardFramework::new()
        .configure(|c| c.prefix("-").with_whitespace(false).owners(owner_ids))
        .group(&GENERALCOMMANDS_GROUP)
        .group(&ADMINCOMMANDS_GROUP)
        .group(&FUNCOMMANDS_GROUP)
        .group(&GAMECOMMANDS_GROUP)
        .group(&CHALLENGECOMMANDS_GROUP)
        .help(&HELP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(fm)
        .await
        .unwrap();

    if let Err(value) = sqlx::migrate!().run(&db).await {
        println!("Version mismatch {:?}", value);
    }

    {
        let mut data = client.data.write().await;
        data.insert::<BotDb>(Arc::new(RwLock::new(db)));
    }
    client.start().await.unwrap();
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
