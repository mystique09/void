use std::sync::Arc;

use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
        ChannelId,
    },
    prelude::Context,
    utils::MessageBuilder,
};
use tracing::{error, info};

use crate::bot::shared::{Guild, SharedGuildState};

pub async fn run(
    ctx: Arc<Context>,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let user = options
        .get(0)
        .expect("no user found")
        .resolved
        .as_ref()
        .expect("no user found");
    let guild_id = command.guild_id.unwrap();

    let data = ctx
        .data
        .read()
        .await
        .get::<SharedGuildState>()
        .unwrap()
        .clone();

    if let CommandDataOptionValue::User(user, _pmember) = user {
        if user.id != command.user.id {
            return "you cannot cancel others bump, that will annoy them".to_string();
        }

        /*
        To cancel a bump, we need to know whether a bump for the user
        already exist, thus we iterate each bump. If no bump exist then
        we immediately return the task. I don't know yet if we can cancel a tokio task,
        I know this is kinda bullshit but hey it works!

        What's the downside of this approach?
        - Even if we delete the bump in the cache,
          the task is still there running(in sleep mode).
          We just have to wait for it, and immediately return.
        */
        let mut guild_state = data.write().await;

        match guild_state.get_mut(&guild_id) {
            Some(guild) => {
                let mut i: isize = -1;

                for bump in guild.bumps.iter() {
                    if bump.0 == user.id {
                        i += 1;
                        break;
                    }
                    i += 1;
                }

                if i == -1 {
                    error!(
                        "cannot cancel bump, no bump found for {}/{}",
                        &user.name, &user.id
                    );

                    MessageBuilder::new()
                        .push("no bump scheduled for ")
                        .user(user.id)
                        .build()
                } else {
                    guild.bumps.remove(i as usize);
                    info!(
                        "bump for {} has been canceled, all running bumps: {:#?}",
                        &user.id, &guild.bumps
                    );

                    MessageBuilder::new()
                        .push("bump canceled for user ")
                        .user(user.id)
                        .build()
                }
            }
            None => {
                let guild_cache = ctx.cache.guild(guild_id).unwrap();
                let channels: Vec<(String, ChannelId)> = guild_cache
                    .channels
                    .into_iter()
                    .map(|c| (c.1.to_string(), c.0))
                    .collect();

                guild_state.insert(
                    guild_id,
                    Guild {
                        keywords: vec![],
                        bumps: vec![],
                        channels,
                    },
                );

                "no bump scheduled for user".to_string()
            }
        }
    } else {
        error!("missing user option when cancling bump");

        "missing user option".to_string()
    }
}

pub fn register(
    command: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
    command
        .name("cancel")
        .description("Cancel the created bump.")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|option| {
            option
                .name("user")
                .description("who's bump needs to cancel")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
