use thiserror::Error;

use void_domain::entity::user::{NickName, UserId};

use crate::gateway::repository::user::{FindOneError, Repo};
use crate::services::user::check_user::Error::NotFound;

#[derive(Debug)]
pub struct Request {
    pub id: UserId,
    pub nickname: NickName,
}

pub struct CheckUser<'r, R> {
    repo: &'r R,
}

impl<'r, R> CheckUser<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", FindOneError::Connection)]
    Repo,
    #[error("{}", FindOneError::NotFound)]
    NotFound(UserId),
}

impl<'r, R> CheckUser<'r, R>
    where
        R: Repo
{
    pub fn exec(&self, req: Request) -> Result<(), Error> {
        match self.repo.find_one(req.id.clone()) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e {
                    FindOneError::Connection => Err(Error::Repo),
                    FindOneError::NotFound => {
                        let id = req.id;
                        Err(NotFound(id))
                    }
                }
            }
        }
    }
}
