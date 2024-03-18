use serenity::all::{Context, Interaction, Ready};
use serenity::async_trait;
use serenity::client::EventHandler;

use crate::commands::slash::{
    register_commands, register_global_commands, register_local_commands, run, CommandScope,
};

pub struct InteractionEventHandler;

#[async_trait]
impl EventHandler for InteractionEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        register_local_commands(&ctx, guild_id).await;

        #[cfg(release)]
        register_global_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let ctx_copy = Arc::new(ctx);

            let response = run(&ctx_copy, &command, &command.data.options).await;

            if let Err(why) = command
                .create_interaction_response(&ctx_copy, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response))
                })
                .await
            {
                println!("error running slash command")
            }
        }
    }
}
