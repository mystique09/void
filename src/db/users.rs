use std::sync::Arc;

use serenity::async_trait;
use sqlx::{error::DatabaseError, postgres::PgPool};

#[mockall::automock]
#[async_trait]
pub trait UserRepo {
    async fn add_user(&self, id: u64) -> Result<u64, Box<dyn DatabaseError>>;
    async fn get_user(&self, id: u64) -> Result<u64, Box<dyn DatabaseError>>;
}

pub struct PostgresUserRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresUserRepo {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn add_user(&self, id: u64) -> Result<u64, Box<dyn DatabaseError>> {
        let query = sqlx::query!(
            r#"
       INSERT INTO "user" (dc_id)
       VALUES ($1)
       RETURNING user_id
       "#,
            id
        )
        .fetch_one(&*self.pg_pool)
        .await;

        query
    }
    async fn get_user(&self, id: u64) -> Result<u64, Box<dyn DatabaseError>> {
        let query = sqlx::query!(
            r#"
        SELECT * FROM "user"
        WHERE id=$1
        "#,
            id
        )
        .fetch_one(&*self.pg_pool)
        .await;

        query
    }
}
