use std::fmt::Debug;
use void_application::gateway::repository as repo;
use void_application::identifier::NewId;

pub trait Database: repo::user::Repo + NewId<void_domain::UserId> + 'static + Debug {}
