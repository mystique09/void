use serenity::async_trait;

use crate::bootstrap::database::Database;
use crate::domain::auto_respond;
use anyhow::Result;
use sqlx::Error as SqlxError;

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
        let query = sqlx::query_as!(
            auto_respond::Keyword,
            r#"
        INSERT INTO "keywords" (
            id, word, response, response_type, response_mode   
        ) VALUES (
            $1, $2, $3, $4, $5
        ) RETURNING *;
        "#,
            data.id,
            data.word,
            data.response,
            data.response_type,
            data.response_mode
        )
        .fetch_one(&self.conn.pool)
        .await;

        query
    }
    async fn get_keyword(&self, id: i64) -> Result<auto_respond::Keyword, SqlxError> {
        todo!()
    }
    async fn get_keywords(&self) -> Result<Vec<auto_respond::Keyword>, SqlxError> {
        todo!()
    }
    async fn update_keyword(&self, id: i64, data: auto_respond::UpdateKeywordDTO) -> Result<bool> {
        todo!()
    }
    async fn delete_keyword(&self, id: i64) -> Result<bool> {
        todo!()
    }
}
