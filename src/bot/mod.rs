pub mod commands;
pub mod events;
pub mod handler;
pub mod init;
pub mod shared;
pub mod ui;

pub const DEFAULT_PREFIX: &str = "!";
pub const LOG_COOLDOWN: u64 = 120;
