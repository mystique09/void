use serenity::async_trait;

use crate::domain::auto_respond;
use crate::repository::keyword_repository;
use anyhow::Result;
use sqlx::Error as SqlxError;

pub struct KeywordUsecase {
    pub repository: keyword_repository::KeywordRepository,
}

impl KeywordUsecase {
    pub fn new(repository: keyword_repository::KeywordRepository) -> Self {
        Self { repository }
    }
}

#[async_trait(?Send)]
impl auto_respond::KeywordUsecase for KeywordUsecase {
    async fn create_keyword(
        &self,
        data: auto_respond::CreateKeywordDTO,
    ) -> Result<auto_respond::Keyword, SqlxError> {
        todo!()
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
