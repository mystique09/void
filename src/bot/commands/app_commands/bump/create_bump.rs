use std::sync::Arc;

use crate::{
    bot::shared::{Guild, SharedGuildState},
    utils::helpers::{generate_random_bump, generate_random_response},
};
use chrono::Duration;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
        ChannelId, GuildId, UserId,
    },
    prelude::{Context, Mentionable},
    utils::MessageBuilder,
};
use tracing::{error, info};

pub async fn run(
    ctx: Arc<Context>,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let user = options
        .get(0)
        .expect("who to bump?")
        .resolved
        .as_ref()
        .expect("who to bump?");
    let guild_id = command.guild_id.unwrap();

    let data = ctx
        .data
        .read()
        .await
        .get::<SharedGuildState>()
        .unwrap()
        .clone();

    if let CommandDataOptionValue::User(user, _pmember) = user {
        let bump_schedule = options
            .get(1)
            .expect("when to bump?")
            .resolved
            .as_ref()
            .expect("when to bump?");

        if user.id != command.user.id {
            return "you cannot do that, that will annoy them".to_string();
        }

        if let CommandDataOptionValue::String(schedule) = bump_schedule {
            let mut guild_state = data.write().await;
            let guild = match guild_state.get_mut(&guild_id) {
                Some(map) => map,
                None => {
                    let bumps: Vec<(UserId, Duration)> = vec![];
                    guild_state.insert(
                        guild_id,
                        Guild {
                            channels: vec![],
                            keywords: vec![],
                            bumps,
                        },
                    );

                    let guild = guild_state.get_mut(&guild_id).unwrap();
                    guild
                }
            };

            if guild.bumps.iter().filter(|b| b.0 == user.id).count() > 0 {
                return MessageBuilder::new()
                    .user(command.user.id)
                    .push(" bump already scheduled, you can cancel it via `/bump_cancel` command")
                    .build();
            }

            let dur = match schedule.as_str() {
                "10s" => Duration::seconds(10),
                "1h" => Duration::hours(1),
                "2h" => Duration::hours(2),
                "5h" => Duration::hours(5),
                "1d" => Duration::days(1),
                "1w" => Duration::weeks(1),
                _ => {
                    return MessageBuilder::new()
                        .user(command.user.id)
                        .push("Invalid choice")
                        .build();
                }
            };

            let dur_name = match schedule.as_str() {
                "10s" => "10 seconds",
                "1h" => "1 hour",
                "2h" => "2 hours",
                "5h" => "5 hours",
                "1d" => "1 day",
                "1w" => "1 week",
                _ => "out of scope",
            };

            guild.bumps.push((user.id, dur));
            info!(
                "new bump created, all running bumps for guild [{}] {:#?}",
                &guild_id, &guild.bumps
            );
            let response = generate_random_bump().await;
            let bump_response = response.to_string().replace("{}", dur_name);

            let ctxcpy = Arc::new(ctx);
            schedule_bump(
                Arc::clone(&ctxcpy),
                &bump_response,
                command.channel_id,
                guild_id,
                user.id,
                dur,
            )
            .await;

            bump_response
        } else {
            "I am not a magician, please provided a schedule.".to_string()
        }
    } else {
        "Who to bump?".to_string()
    }
}

async fn schedule_bump(
    ctx: Arc<Context>,
    response: &str,
    channel_id: ChannelId,
    guild_id: GuildId,
    user_id: UserId,
    dur: Duration,
) {
    let data = ctx
        .data
        .read()
        .await
        .get::<SharedGuildState>()
        .unwrap()
        .clone();

    info!("{}", response);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(dur.num_seconds() as u64)).await;
        let mut guild_state = data.write().await;
        let guild = guild_state.get_mut(&guild_id).unwrap();

        /*
        To cancel a bump, we need to know whether a bump for the user
        already exist, thus we iterate each bump. If no bump exist then
        we immediately return the task. I don't know yet if we can cancel a tokio task,
        I know this is kinda bullshit but hey it works!
        */
        let mut i: isize = -1;

        for bump in guild.bumps.iter() {
            if bump.0 == user_id {
                i += 1;
                break;
            }
            i += 1;
        }

        // if bump is deleted then we immediately return the async task
        if i == -1 {
            return;
        }

        // else remove the bump in cache and bump the user
        guild.bumps.remove(i as usize);
        let response = generate_random_response().await;
        info!(
            "{} {member}, all running bumps for guild [{}]: {:#?}",
            &response,
            &guild_id,
            &guild.bumps,
            member = &user_id.mention(),
        );

        let message = channel_id
            .send_message(&ctx.http, |message| {
                message.content(format!(
                    "{} {member}.",
                    response,
                    member = user_id.mention()
                ))
            })
            .await;

        if let Err(why) = message {
            error!("Error while bumping user: {}", why);
        }
    });
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bump")
        .description("Will bump/remind you.")
        .create_option(|option| {
            option
                .name("who")
                .description("who to bump")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("time")
                .description("when to bump")
                .kind(CommandOptionType::String)
                .add_string_choice("10 seconds", "10s")
                .add_string_choice("1 hour", "1h")
                .add_string_choice("2 hours", "2h")
                .add_string_choice("5 hours", "5h")
                .add_string_choice("1 day", "1d")
                .add_string_choice("1 week", "1w")
                .required(true)
        })
}
