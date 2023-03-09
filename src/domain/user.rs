use anyhow::Result;
use chrono::NaiveDate;
use serenity::async_trait;
use sqlx::Error as SqlxError;

pub struct User {
    pub id: i64,
    pub nickname: String,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
}

pub struct CreateUserDTO {
    pub id: i64,
    pub nickname: String,
}

pub enum UserError {
    UserNotFound,
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, data: CreateUserDTO) -> Result<User, SqlxError>;
    async fn get_users(&self) -> Result<Vec<User>, SqlxError>;
    async fn get_user_by_id(&self, id: i64) -> Result<User, SqlxError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, SqlxError>;
    async fn set_user_exp(&self, new_exp: i32) -> Result<bool>;
    async fn delete_user(&self, id: i64) -> Result<bool>;
}

#[async_trait]
pub trait UserUsecase {
    async fn create_user(&self, data: CreateUserDTO) -> Result<User, SqlxError>;
    async fn get_users(&self) -> Result<Vec<User>, SqlxError>;
    async fn get_user_by_id(&self, id: i64) -> Result<User, SqlxError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, SqlxError>;
    async fn set_user_exp(&self, new_exp: i32) -> Result<bool>;
    async fn delete_user(&self, id: i64) -> Result<bool>;
}
