use std::sync::Arc;

use serenity::async_trait;
use sqlx::{postgres::PgPool, Executor};

#[derive(Debug)]
pub struct User {
    dc_id: i64,
    user_rank: Option<i32>,
    user_balance: Option<u32>,
}

#[mockall::automock]
#[async_trait]
pub trait UserRepo {
    async fn get_users(&self) -> anyhow::Result<i64>;
    async fn get_user(&self, id: i64) -> anyhow::Result<i64>;
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
    async fn get_users(&self) -> anyhow::Result<i64> {
        let _rec = sqlx::query!(
            r#"SELECT dc_id, user_rank, user_balance
        FROM "user" ORDER BY dc_id"#
        )
        .fetch_all(&*self.pg_pool)
        .await?;

        Ok(2)
    }
    async fn get_user(&self, _id: i64) -> anyhow::Result<i64> {
        let rec = sqlx::query!(
            r#"
        SELECT dc_id, user_rank, user_balance
        FROM "user"
        WHERE dc_id=$1
        "#,
            _id
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(rec.dc_id)
    }
}
