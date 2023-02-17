use crate::{bootstrap::database::Database, domain};
use anyhow::Result;
use serenity::async_trait;
use sqlx::Error as SqlxError;

pub struct UserRepository {
    pub conn: Database,
}

impl UserRepository {
    pub fn new(conn: Database) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl domain::user::UserRepository for UserRepository {
    async fn create_user(
        &self,
        data: domain::user::CreateUserDTO,
    ) -> Result<domain::user::User, SqlxError> {
        let query = sqlx::query_as!(
            domain::user::User,
            r#"
        INSERT INTO "users" (
            id, nickname
        ) VALUES (
            $1, $2
        )
        RETURNING *
        "#,
            data.id,
            data.nickname
        )
        .fetch_one(&self.conn.pool)
        .await;

        query
    }

    async fn get_users(&self) -> Result<Vec<domain::user::User>, SqlxError> {
        let query = sqlx::query_as!(
            domain::user::User,
            r#"
        SELECT * FROM "users"
        "#
        )
        .fetch_all(&self.conn.pool)
        .await;

        query
    }

    async fn get_user_by_id(&self, id: i64) -> Result<domain::user::User, SqlxError> {
        let query = sqlx::query_as!(
            domain::user::User,
            r#"
            SELECT * FROM "users"
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.conn.pool)
        .await;
        query
    }

    async fn get_user_by_username(&self, username: &str) -> Result<domain::user::User, SqlxError> {
        let query = sqlx::query_as!(
            domain::user::User,
            r#"
            SELECT * FROM "users"
            WHERE nickname = $1
            "#,
            username
        )
        .fetch_one(&self.conn.pool)
        .await;
        query
    }

    async fn set_user_exp(&self, _new_exp: i32) -> Result<bool> {
        todo!()
    }

    async fn delete_user(&self, id: i64) -> Result<bool> {
        let query = sqlx::query!(
            r#"
                DELETE FROM "users"
                WHERE id = $1
                "#,
            id
        )
        .execute(&__self.conn.pool)
        .await?
        .rows_affected();

        Ok(query > 0)
    }
}
