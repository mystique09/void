use serenity::all::standard::macros::group;

mod pwdgen;

use pwdgen::PWDGEN_COMMAND;

#[group]
#[description = "General commands that users can use."]
#[summary = "Basic commands."]
#[commands(pwdgen)]
pub struct GeneratorCommands;