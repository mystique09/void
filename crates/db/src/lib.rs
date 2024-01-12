use sqlx::PgPool;

use void_adapter::db::Database;

#[derive(Debug)]
pub struct CloudDatabase {
    pub pool: PgPool,
}

impl CloudDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Database for CloudDatabase {}

mod user {
    use void_application::gateway::repository::user::{DeleteOneError, FindAllError, FindOneError, Map, Repo, SaveError};
    use void_application::identifier::{NewId, NewIdError};
    use void_domain::UserId;

    use super::*;

    impl Repo for CloudDatabase {
        fn save(&self, map: Map) -> Result<(), SaveError> {
            todo!("{:?}", map)
        }

        fn find_one(&self, id: UserId) -> Result<Map, FindOneError> {
            todo!("{:?}", id)
        }

        fn find_all(&self) -> Result<Vec<Map>, FindAllError> {
            todo!()
        }

        fn delete_one(&self, id: UserId) -> Result<(), DeleteOneError> {
            todo!("{:?}", id)
        }
    }

    impl NewId<UserId> for CloudDatabase {
        fn new(&self) -> Result<UserId, NewIdError> {
            todo!()
        }
    }
}