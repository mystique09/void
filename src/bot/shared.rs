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
type BumpMap = HashMap<GuildId, Vec<(UserId, Duration)>>;
impl TypeMapKey for SharedBumpState {
    type Value = Arc<RwLock<BumpMap>>;
}

pub struct SharedKeywordsState;
type KeywordMap = HashMap<GuildId, Vec<crate::domain::auto_respond::Keyword>>;
impl TypeMapKey for SharedKeywordsState {
    type Value = Arc<RwLock<KeywordMap>>;
}
