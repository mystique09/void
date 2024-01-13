use serenity::framework::standard::macros::group;

// serenity auto-generates a CAPITALIZED function based on the function name + COMMAND suffix
use info::INFO_COMMAND;

pub mod info;

#[group]
#[description = "General commands that users can use."]
#[summary = "Basic commands."]
#[commands(info)]
pub struct GeneralCommands;