use anyhow::Result;
use serenity::async_trait;
use sqlx::Error as SqlxError;

use crate::domain;

pub struct UserUsecase<'a> {
    pub repository: &'a dyn domain::user::UserRepository,
}

impl<'a> UserUsecase<'a> {
    pub fn new(repository: &'a dyn domain::user::UserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait(?Send)]
impl<'a> domain::user::UserUsecase for UserUsecase<'a> {
    async fn create_user(
        &self,
        data: domain::user::CreateUserDTO,
    ) -> Result<domain::user::User, SqlxError> {
        self.repository.create_user(data).await
    }

    async fn get_users(&self) -> Result<Vec<domain::user::User>, SqlxError> {
        self.repository.get_users().await
    }

    async fn get_user_by_id(&self, id: i64) -> Result<domain::user::User, SqlxError> {
        self.repository.get_user_by_id(id).await
    }

    async fn get_user_by_username(&self, username: &str) -> Result<domain::user::User, SqlxError> {
        self.repository.get_user_by_username(username).await
    }

    async fn set_user_exp(&self, new_exp: i32) -> Result<bool> {
        self.repository.set_user_exp(new_exp).await
    }

    async fn delete_user(&self, id: i64) -> Result<bool> {
        self.repository.delete_user(id).await
    }
}
