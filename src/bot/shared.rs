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

pub struct SharedKeywordUsecase;

impl TypeMapKey for SharedKeywordUsecase {
    type Value = Arc<RwLock<crate::usecase::keyword_usecase::KeywordUsecase>>;
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

pub struct SharedKeywordsState;

impl TypeMapKey for SharedKeywordsState {
    type Value = Arc<RwLock<Vec<crate::domain::auto_respond::Keyword>>>;
}
