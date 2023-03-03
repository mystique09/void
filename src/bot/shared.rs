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

#[derive(Debug)]
pub struct Usecase {
    pub user_usecase: crate::usecase::user_usecase::UserUsecase,
    pub keywords_usecase: crate::usecase::auto_respond_usecase::KeywordUsecase,
}

#[derive(Debug)]
pub struct SharedUsecase;
impl TypeMapKey for SharedUsecase {
    type Value = Arc<RwLock<Usecase>>;
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
