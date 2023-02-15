use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    utils::Colour,
};

#[group]
#[description = "Group of general commands."]
#[summary = "General commands."]
#[commands(info)]
pub struct GeneralCommands;

#[command]
#[description = "Get the bot info."]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    let user_tmstmp = msg.timestamp;
    let now = Utc::now();
    let ping = (now - user_tmstmp).num_milliseconds();
    let emoji = match ping {
        ping if ping < 100 => "🐆🐆🐆",
        ping if ping > 100 && ping < 500 => "🐆🐆",
        ping if ping > 500 && ping < 1000 => "🐆",
        ping if ping > 1000 && ping < 1100 => "🐢",
        ping if ping > 1100 && ping < 1500 => "🐢🐢",
        _ => "🐢🐢🐢",
    };
    let author = &msg.author.name;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Bot Latency")
                    .color(Colour::BLUE)
                    .field("Latency", format!("{ping} ms"), true)
                    .field("Speed Indicator", emoji, true)
                    .field(
                        "CPU Load Average",
                        format!("{:.2}%", cpu_load.one * 10.0),
                        false,
                    )
                    .field(
                        "Memory Usage",
                        format!(
                            "{:.2} MB Free out of {:.2} MB",
                            mem_use.free as f32 / 1000.0,
                            mem_use.total as f32 / 1000.0
                        ),
                        false,
                    )
                    .author(|a| a.name(author))
            })
        })
        .await?;

    Ok(())
}
