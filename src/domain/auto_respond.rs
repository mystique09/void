use anyhow::Result;
use chrono::NaiveDate;
use serenity::async_trait;
use sqlx::Error as SqlxError;

pub enum ResponseType {
    SingleLine,
    MultiLine,
    Media,
}

pub enum ResponseMode {
    Regular,
    DirectMessage,
}

pub struct Keyword {
    pub id: i64,
    pub word: String,
    pub response: String,
    pub response_type: ResponseType,
    pub response_mode: ResponseMode,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
}

pub struct CreateKeywordDTO {
    pub id: i64,
    pub word: String,
    pub response: String,
    pub response_type: ResponseType,
    pub response_mode: ResponseMode,
}

pub struct UpdateKeywordDTO {
    pub word: String,
    pub response: String,
    pub response_type: Option<ResponseType>,
    pub response_mode: Option<ResponseMode>,
}

#[async_trait]
pub trait KeywordRepository {
    async fn create_keyword(&self, data: CreateKeywordDTO) -> Result<Keyword, SqlxError>;
    async fn get_keyword(&self, id: i64) -> Result<Keyword, SqlxError>;
    async fn get_keywords(&self) -> Result<Vec<Keyword>, SqlxError>;
    async fn update_keyword(&self, id: i64, data: UpdateKeywordDTO) -> Result<bool>;
    async fn delete_keyword(&self, id: i64) -> Result<bool>;
}

#[async_trait(?Send)]
pub trait KeywordUsecase {
    async fn create_keyword(&self, data: CreateKeywordDTO) -> Result<Keyword, SqlxError>;
    async fn get_keyword(&self, id: i64) -> Result<Keyword, SqlxError>;
    async fn get_keywords(&self) -> Result<Vec<Keyword>, SqlxError>;
    async fn update_keyword(&self, id: i64, data: UpdateKeywordDTO) -> Result<bool>;
    async fn delete_keyword(&self, id: i64) -> Result<bool>;
}
