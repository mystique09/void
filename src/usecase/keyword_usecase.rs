use serenity::async_trait;

use crate::domain::auto_respond::{self, KeywordRepository};
use anyhow::Result;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub struct KeywordUsecase {
    pub repository: crate::repository::keyword_repository::KeywordRepository,
}

impl KeywordUsecase {
    pub fn new(repository: crate::repository::keyword_repository::KeywordRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl auto_respond::KeywordUsecase for KeywordUsecase {
    async fn create_keyword(
        &self,
        data: auto_respond::CreateKeywordDTO,
    ) -> Result<auto_respond::Keyword, SqlxError> {
        self.repository.create_keyword(data).await
    }

    async fn get_keyword(&self, id: uuid::Uuid) -> Result<auto_respond::Keyword, SqlxError> {
        self.repository.get_keyword(id).await
    }

    async fn get_keywords(&self, guild_id: i64) -> Result<Vec<auto_respond::Keyword>, SqlxError> {
        self.repository.get_keywords(guild_id).await
    }

    async fn update_response(&self, id: uuid::Uuid, new_response: &str) -> Result<bool> {
        self.repository.update_response(id, new_response).await
    }

    async fn update_response_type(
        &self,
        id: uuid::Uuid,
        new_response_type: auto_respond::ResponseType,
    ) -> Result<bool> {
        self.repository
            .update_response_type(id, new_response_type)
            .await
    }

    async fn update_response_mode(
        &self,
        id: uuid::Uuid,
        new_response_mode: auto_respond::ResponseMode,
    ) -> Result<bool> {
        self.repository
            .update_response_mode(id, new_response_mode)
            .await
    }

    async fn delete_keyword(&self, id: uuid::Uuid) -> Result<bool> {
        self.repository.delete_keyword(id).await
    }
}
