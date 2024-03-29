use serenity::async_trait;

use crate::bootstrap::database::Database;
use crate::domain::auto_respond;
use anyhow::Result;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub struct KeywordRepository {
    pub conn: Database,
}

impl KeywordRepository {
    pub fn new(conn: Database) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl auto_respond::KeywordRepository for KeywordRepository {
    async fn create_keyword(
        &self,
        data: auto_respond::CreateKeywordDTO,
    ) -> Result<auto_respond::Keyword, SqlxError> {
        let query = sqlx::query_as!(auto_respond::Keyword,
            r#"
        INSERT INTO "keywords" (
            id, word, guild_id, response, response_type, response_mode   
        ) VALUES (
            $1, $2, $3, $4, $5, $6
        ) RETURNING id, word, guild_id, response, response_type as "response_type: _", response_mode as "response_mode: _", created_at, updated_at;
        "#,
            data.id,
            data.word,
            data.guild_id,
            data.response,
            data.response_type as _,
            data.response_mode as _
        )
        .fetch_one(&self.conn.pool)
        .await;

        query
    }

    async fn get_keyword(&self, id: uuid::Uuid) -> Result<auto_respond::Keyword, SqlxError> {
        let query = sqlx::query_as!(
            auto_respond::Keyword,
            r#"
        SELECT id, word, guild_id, response, response_type as "response_type: _", response_mode as "response_mode: _", created_at, updated_at
        FROM "keywords"
        WHERE id = $1;
            "#,
            id
        )
        .fetch_one(&self.conn.pool)
        .await;

        query
    }

    async fn get_keywords(&self, guild_id: i64) -> Result<Vec<auto_respond::Keyword>, SqlxError> {
        let query = sqlx::query_as!(
            auto_respond::Keyword,
            r#"
        SELECT id, word, guild_id, response, response_type as "response_type: _", response_mode as "response_mode: _", created_at, updated_at
        FROM "keywords"
        WHERE guild_id = $1;
            "#,
            guild_id
        )
        .fetch_all(&self.conn.pool)
        .await;

        query
    }

    async fn update_response(&self, id: uuid::Uuid, new_response: &str) -> Result<bool> {
        let query = sqlx::query!(
            r#"
            UPDATE "keywords"
            SET response = $2
            WHERE id = $1;
            "#,
            id,
            new_response
        )
        .execute(&self.conn.pool)
        .await?
        .rows_affected();

        Ok(query > 0)
    }

    async fn update_response_type(
        &self,
        id: uuid::Uuid,
        new_response_type: auto_respond::ResponseType,
    ) -> Result<bool> {
        let query = sqlx::query!(
            r#"
            UPDATE "keywords"
            SET response_type = $2
            WHERE id = $1;
            "#,
            id,
            new_response_type as _
        )
        .execute(&self.conn.pool)
        .await?
        .rows_affected();

        Ok(query > 0)
    }

    async fn update_response_mode(
        &self,
        id: uuid::Uuid,
        new_response_mode: auto_respond::ResponseMode,
    ) -> Result<bool> {
        let query = sqlx::query!(
            r#"
            UPDATE "keywords"
            SET response_mode = $2
            WHERE id = $1;
            "#,
            id,
            new_response_mode as _
        )
        .execute(&self.conn.pool)
        .await?
        .rows_affected();

        Ok(query > 0)
    }

    async fn delete_keyword(&self, id: uuid::Uuid) -> Result<bool> {
        let query = sqlx::query!(
            r#"
            DELETE FROM "keywords"
            WHERE id = $1;
            "#,
            id
        )
        .execute(&self.conn.pool)
        .await?
        .rows_affected();

        Ok(query > 0)
    }
}
