use sqlx::PgPool;

use void_adapter::db::Database;
use void_domain;

#[derive(Debug)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Database for PostgresDatabase {}

mod user {
    use tokio::runtime::Runtime;

    use void_application::gateway::repository::user::{DeleteOneError, FindAllError, FindOneError, Map, Repo, SaveError};
    use void_application::identifier::{NewId, NewIdError};
    use void_domain::UserId;

    use super::*;

    impl Repo for PostgresDatabase {
        fn save(&self, map: Map) -> Result<(), SaveError> {
            let rt = Runtime::new().expect("tokio runtime");

            let query = sqlx::query!(
                r#"
                INSERT INTO "users" (
                    id, nickname
                ) VALUES (
                    $1, $2
                );
                "#,
                map.user.id(),
                map.user.nickname()
            );

            let _ = rt.block_on(query.execute(&self.pool))
                .map_err(|e| {
                    if e.to_string().contains("duplicate") {
                        return SaveError::UserAlreadyExist;
                    }

                    return SaveError::Connection;
                })?;

            Ok(())
        }

        fn find_one(&self, id: UserId) -> Result<void_domain::User, FindOneError> {
            let rt = Runtime::new().expect("tokio runtime");

            let query = sqlx::query_as!(
                void_domain::User,
                r#"
                    SELECT * FROM "users"
                    WHERE id = $1;
                "#,
                id.to_i64()
            );

            let user = rt.block_on(query.fetch_one(&self.pool))
                .map_err(|_| FindOneError::NotFound)?;

            Ok(user)
        }

        fn find_all(&self) -> Result<Vec<void_domain::User>, FindAllError> {
            let rt = Runtime::new().expect("tokio runtime");

            let query = sqlx::query_as!(
               void_domain::User,
                r#"
                    SELECT * FROM "users"
                    GROUP BY id;
                "#
            );

            let users = rt.block_on(query.fetch_all(&self.pool))
                .map_err(|_| FindAllError::Connection)?;

            Ok(users)
        }

        fn delete_one(&self, id: UserId) -> Result<(), DeleteOneError> {
            let rt = Runtime::new().expect("tokio runtime");

            let query = sqlx::query!(r#"DELETE FROM "users" WHERE id = $1;"#, id.to_i64());

            let _ = rt.block_on(query.execute(&self.pool))
                .map_err(|e| {
                    match e {
                        sqlx::Error::RowNotFound => DeleteOneError::NotFound,
                        _ => DeleteOneError::Connection
                    }
                })?;

            Ok(())
        }
    }

    impl NewId<UserId> for PostgresDatabase {
        fn new(&self) -> Result<UserId, NewIdError> {
            todo!()
        }
    }
}