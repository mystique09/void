use thiserror::Error;

use void_domain::{entity::user::UserId, User};

#[derive(Debug, Error)]
pub enum FindOneError {
    #[error("User not found")]
    NotFound,
    #[error("User repository disconnected")]
    Connection,
}

#[derive(Debug, Error)]
pub enum FindAllError {
    #[error("User repository disconnected")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("User repository disconnected")]
    Connection,
    #[error("User already exist")]
    UserAlreadyExist,
}

#[derive(Debug, Error)]
pub enum DeleteOneError {
    #[error("User not found")]
    NotFound,
    #[error("User repository disconnected")]
    Connection,
}

#[derive(Debug)]
pub struct Map {
    pub user: User,
}

pub trait Repo: Send + Sync {
    fn save(&self, map: Map) -> Result<(), SaveError>;
    fn find_one(&self, id: UserId) -> Result<User, FindOneError>;
    fn find_all(&self) -> Result<Vec<User>, FindAllError>;
    fn delete_one(&self, id: UserId) -> Result<(), DeleteOneError>;
}
