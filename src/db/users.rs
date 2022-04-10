use std::sync::Arc;

use serenity::async_trait;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct User {
    dc_bot: i64,
    user_rank: Option<i32>,
    user_balance: Option<u32>,
}

#[mockall::automock]
#[async_trait]
pub trait UserRepo {
    async fn get_users() -> anyhow::Result<i64>;
    async fn get_user(id: u64) -> anyhow::Result<i64>;
}

pub struct PgRepo {
    pub pg_pool: Arc<PgPool>,
}

impl PgRepo {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl UserRepo for PgRepo {
    async fn get_users() -> anyhow::Result<i64> {
        Ok(2)
    }
    async fn get_user(_id: u64) -> anyhow::Result<i64> {
        Ok(2)
    }
}
