use std::sync::Arc;

use chrono::Duration;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        ChannelId, UserId,
    },
    prelude::{Context, Mentionable},
};
use tracing::{error, info};

use crate::bot::shared::SharedBumpState;

pub async fn run(ctx: Arc<Context>, options: &[CommandDataOption]) -> String {
    let user = options
        .get(0)
        .expect("who to bump?")
        .resolved
        .as_ref()
        .expect("who to bump?");

    let data = ctx
        .data
        .read()
        .await
        .get::<SharedBumpState>()
        .unwrap()
        .clone();

    if let CommandDataOptionValue::User(user, _pmember) = user {
        let bump_schedule = options
            .get(1)
            .expect("when to bump?")
            .resolved
            .as_ref()
            .expect("when to bump?");

        if let CommandDataOptionValue::String(schedule) = bump_schedule {
            let mut bumps_cache = data.write().await;

            if bumps_cache.iter().filter(|b| b.0 == user.id).count() > 0 {
                return "some already bumped that user".to_string();
            }

            let dur = match schedule.as_str() {
                "10s" => Duration::seconds(10),
                "1h" => Duration::hours(1),
                "2h" => Duration::hours(2),
                "5h" => Duration::hours(5),
                "1d" => Duration::days(1),
                "1w" => Duration::weeks(1),
                _ => {
                    return "invalid time".to_string();
                }
            };

            bumps_cache.push((user.id, dur));
            info!("Total running bumps: {}", bumps_cache.len());

            let ctxcpy = Arc::new(ctx);
            schedule_bump(Arc::clone(&ctxcpy), user.id, dur).await;

            format!("Uhh, ok. Bump {} after {}", &user.name, schedule)
        } else {
            "I am not a magician, please provided a schedule.".to_string()
        }
    } else {
        "Who to bump?".to_string()
    }
}

async fn schedule_bump(ctx: Arc<Context>, user_id: UserId, dur: Duration) {
    let data = ctx
        .data
        .read()
        .await
        .get::<SharedBumpState>()
        .unwrap()
        .clone();

    info!("Uhh, ok. Bump {} after {}", user_id.mention(), dur);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(dur.num_seconds() as u64)).await;
        let mut bumps_cache = data.write().await;

        let mut i = 0;

        for bump in bumps_cache.iter() {
            if bump.0 == user_id {
                break;
            }
            i += 1;
        }

        bumps_cache.remove(i);

        let message = ChannelId(1076420188364345435)
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Times up!").field(
                        "Done",
                        format!("Bump {}, welcome back to reality!", user_id.mention()),
                        false,
                    )
                })
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
