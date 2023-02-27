use std::sync::Arc;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
    },
    prelude::Context,
    utils::MessageBuilder,
};
use tracing::{error, info};

use crate::bot::shared::SharedBumpState;

pub async fn run(
    ctx: Arc<Context>,
    _command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let user = options
        .get(0)
        .expect("no user found")
        .resolved
        .as_ref()
        .expect("no user found");

    let data = ctx
        .data
        .read()
        .await
        .get::<SharedBumpState>()
        .unwrap()
        .clone();

    if let CommandDataOptionValue::User(user, _pmember) = user {
        let mut bumps_cache = data.write().await;

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
        let mut i: isize = -1;

        for bump in bumps_cache.iter() {
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
                .user(user.id)
                .push(" no bump scheduled for this user")
                .build()
        } else {
            bumps_cache.remove(i as usize);
            info!("bump for {} has been canceled", &user.id);
            MessageBuilder::new()
                .user(user.id)
                .push(" bump canceled for this user")
                .build()
        }
    } else {
        error!("missing user option when cancling bump");
        "missing user option".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bump_cancel")
        .description("Cancel the created bump.")
        .create_option(|option| {
            option
                .name("user")
                .description("who's bump needs to cancel")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
