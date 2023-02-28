use anyhow::Result;
use chrono::NaiveDate;
use serenity::async_trait;
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "ResponseType", rename_all = "lowercase")]
pub enum ResponseType {
    SingleLine,
    MultiLine,
    Media,
}

impl From<&str> for ResponseType {
    fn from(v: &str) -> Self {
        match v {
            "SINGLE" => Self::SingleLine,
            "MULTI" => Self::MultiLine,
            "MEDIA" => Self::Media,
            _ => panic!("not supported"),
        }
    }
}

impl From<String> for ResponseType {
    fn from(v: String) -> Self {
        match &*v {
            "SINGLE" => Self::SingleLine,
            "MULTI" => Self::MultiLine,
            "MEDIA" => Self::Media,
            _ => panic!("not supported"),
        }
    }
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleLine => write!(f, "SINGLE"),
            Self::MultiLine => write!(f, "MULTI"),
            Self::Media => write!(f, "MEDIA"),
        }
    }
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "ResponseMode", rename_all = "lowercase")]
pub enum ResponseMode {
    Regular,
    DirectMessage,
}

impl From<&str> for ResponseMode {
    fn from(v: &str) -> Self {
        match v {
            "REGULAR" => Self::Regular,
            "DM" => Self::DirectMessage,
            _ => panic!("not supported"),
        }
    }
}

impl From<String> for ResponseMode {
    fn from(v: String) -> Self {
        match &*v {
            "REGULAR" => Self::Regular,
            "DM" => Self::DirectMessage,
            _ => panic!("not supported"),
        }
    }
}

impl fmt::Display for ResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "REGULAR"),
            Self::DirectMessage => write!(f, "DM"),
        }
    }
}

#[derive(Debug)]
pub struct Keyword {
    pub id: i64,
    pub word: String,
    pub response: String,
    pub response_type: ResponseType,
    pub response_mode: ResponseMode,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
}

impl From<CreateKeywordDTO> for Keyword {
    fn from(data: CreateKeywordDTO) -> Self {
        Self {
            id: data.id,
            word: data.word,
            response: data.response,
            response_type: data.response_type,
            response_mode: data.response_mode,
            created_at: None,
            updated_at: None,
        }
    }
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
    async fn update_response(&self, id: i64, new_response: &str) -> Result<bool>;
    async fn update_response_type(&self, id: i64, new_response_type: ResponseType) -> Result<bool>;
    async fn update_response_mode(&self, id: i64, new_response_mode: ResponseMode) -> Result<bool>;
    async fn delete_keyword(&self, id: i64) -> Result<bool>;
}

#[async_trait]
pub trait KeywordUsecase {
    async fn create_keyword(&self, data: CreateKeywordDTO) -> Result<Keyword, SqlxError>;
    async fn get_keyword(&self, id: i64) -> Result<Keyword, SqlxError>;
    async fn get_keywords(&self) -> Result<Vec<Keyword>, SqlxError>;
    async fn update_response(&self, id: i64, new_response: &str) -> Result<bool>;
    async fn update_response_type(&self, id: i64, new_response_type: ResponseType) -> Result<bool>;
    async fn update_response_mode(&self, id: i64, new_response_mode: ResponseMode) -> Result<bool>;
    async fn delete_keyword(&self, id: i64) -> Result<bool>;
}
