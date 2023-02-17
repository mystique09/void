use std::{collections::HashMap, sync::Arc};

use chrono::Duration;
use serenity::{
    model::prelude::{ChannelId, GuildId, UserId},
    prelude::{RwLock, TypeMapKey},
};

// use crate::bootstrap::database::Database;

// #[derive(Debug)]
// pub struct SharedState;

// impl TypeMapKey for SharedState {
//     type Value = Arc<RwLock<Database>>;
// }

pub struct SharedUserUsecase;

impl TypeMapKey for SharedUserUsecase {
    type Value = Arc<RwLock<crate::usecase::user_usecase::UserUsecase>>;
}

#[derive(Debug)]
pub struct SharedGuildState;

#[derive(Debug)]
pub struct Guild {
    pub channels: Vec<(String, ChannelId)>,
}

impl TypeMapKey for SharedGuildState {
    type Value = Arc<RwLock<HashMap<GuildId, Guild>>>;
}

#[derive(Debug)]
pub struct SharedBumpState;

impl TypeMapKey for SharedBumpState {
    type Value = Arc<RwLock<Vec<(UserId, Duration)>>>;
}
