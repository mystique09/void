use thiserror::Error;

use void_domain::entity::user::{NickName, UserId};
use void_domain::User;

use crate::gateway::repository::user::{Map, Repo, SaveError};
use crate::identifier::{NewId, NewIdError};
use crate::services::user::validator;
use crate::services::user::validator::{UserInvalidity, validate_user_fields};

#[derive(Debug)]
pub struct Request(String);

#[derive(Debug)]
pub struct Response(UserId);

pub struct CreateUser<'r, 'g, R, G> {
    repo: &'r R,
    id_generator: &'g G,
}

impl<'r, 'g, R, G> CreateUser<'r, 'g, R, G> {
    pub fn new(repo: &'r R, id_generator: &'g G) -> Self {
        Self { repo, id_generator }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] UserInvalidity),
}

impl From<SaveError> for Error {
    fn from(value: SaveError) -> Self {
        match value {
            SaveError::Connection => Self::Repo
        }
    }
}

impl<'r, 'g, R, G> CreateUser<'r, 'g, R, G>
    where
        R: Repo,
        G: NewId<UserId>
{
    pub async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Create new user: {:?}", req);
        validate_user_fields(&validator::Request { nick_name: req.0.as_str() })?;
        let nickname = NickName::new(req.0);
        let id = self.id_generator.new().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let user = User::new(id.clone(), nickname);
        let map = Map {
            user
        };
        self.repo.save(map)?;
        Ok(Response(id.clone()))
    }
}