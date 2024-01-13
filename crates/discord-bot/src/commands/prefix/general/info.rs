use chrono::Utc;
use serenity::all::{Context, Message};
use serenity::all::Color;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

use crate::helpers::system::{cpu_usage, memory_usage};

#[command]
#[description = "Will fetch the bot info including the memory usage of the bot."]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let cpu_usage = cpu_usage().await;
    let mem_usage = memory_usage().await;

    let cpu_load = format!("{:.2}%", cpu_usage);
    let memory_load = format!("{:.2} MB free out of {:.2}", mem_usage.0, mem_usage.1);

    let author = &msg.author.name;
    let user_timestamp = msg.timestamp;
    let now = Utc::now();
    let ping = now.timestamp_millis() - user_timestamp.timestamp_millis();
    let emoji = get_matching_emoji(ping);

    msg.channel_id
        .send_message(&ctx.http, CreateMessage::new()
            .embed(CreateEmbed::new()
                .title("Bot Information")
                .color(Color::BLUE)
                .author(CreateEmbedAuthor::new(author))
                .field("Ping", format!("{} ms", ping), true)
                .field("Ping indicator", emoji, true)
                .field("CPU usage", cpu_load, false)
                .field("Memory usage", memory_load, false)
            ),
        )
        .await?;

    Ok(())
}

fn get_matching_emoji<'a>(ping: i64) -> &'a str {
    match ping {
        ping if ping <= 100 => "🐆🐆🐆",
        ping if ping > 100 && ping <= 500 => "🐆🐆",
        ping if ping > 500 && ping <= 1000 => "🐆",
        ping if ping > 1000 && ping <= 1100 => "🐢",
        ping if ping > 1100 && ping <= 1500 => "🐢🐢",
        _ => "🐢🐢🐢",
    }
}