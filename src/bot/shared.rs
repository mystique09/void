use std::{collections::HashMap, sync::Arc};

use chrono::Duration;
use serenity::{
    model::prelude::{ChannelId, GuildId, UserId},
    prelude::{RwLock, TypeMapKey},
};

use crate::bootstrap::env::Env;

pub struct SharedEnvState;

impl TypeMapKey for SharedEnvState {
    type Value = Arc<RwLock<Env>>;
}

pub struct SharedUserUsecase;

impl TypeMapKey for SharedUserUsecase {
    type Value = Arc<RwLock<crate::usecase::user_usecase::UserUsecase>>;
}

#[derive(Debug)]
pub struct SharedKeywordUsecase;

impl TypeMapKey for SharedKeywordUsecase {
    type Value = Arc<RwLock<crate::usecase::keyword_usecase::KeywordUsecase>>;
}
#[derive(Debug)]
pub struct Guild {
    pub channels: Vec<(String, ChannelId)>,
    pub bumps: Vec<(UserId, Duration)>,
    pub keywords: Vec<crate::domain::auto_respond::Keyword>,
}

#[derive(Debug)]
pub struct SharedGuildState;

impl TypeMapKey for SharedGuildState {
    type Value = Arc<RwLock<HashMap<GuildId, Guild>>>;
}
