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
use tracing::error;

#[group]
#[description = "Group of general commands."]
#[summary = "General commands."]
#[commands(info)]
pub struct GeneralCommands;

#[command]
#[description = "Get the bot info."]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let cpu_load = match sys_info::loadavg() {
        Ok(cpu) => format!("{:.2}%", cpu.one * 10.0),
        Err(why) => {
            error!("{}", why);
            "not available".to_string()
        }
    };
    let mem_use = match sys_info::mem_info() {
        Ok(mem) => format!(
            "{:.2} MB Free out of {:.2}",
            mem.free as f32 / 1000.0,
            mem.total as f32 / 1000.0
        ),
        Err(why) => {
            error!("{}", why);
            "not available".to_string()
        }
    };

    let user_tmstmp = msg.timestamp;
    let now = Utc::now();
    let ping = now.timestamp_millis() - user_tmstmp.timestamp_millis();
    let emoji = match ping {
        ping if ping <= 100 => "🐆🐆🐆",
        ping if ping > 100 && ping <= 500 => "🐆🐆",
        ping if ping > 500 && ping <= 1000 => "🐆",
        ping if ping > 1000 && ping <= 1100 => "🐢",
        ping if ping > 1100 && ping <= 1500 => "🐢🐢",
        _ => "🐢🐢🐢",
    };
    let author = &msg.author.name;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Bot Latency")
                    .color(Colour::BLUE)
                    .field("Latency", format!("{} ms", ping), true)
                    .field("Speed Indicator", emoji, true)
                    .field("CPU Load Average", cpu_load, false)
                    .field("Memory Usage", mem_use, false)
                    .author(|a| a.name(author))
            })
        })
        .await?;

    Ok(())
}
