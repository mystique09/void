use sqlx::PgPool;
use tracing::log;

use void_db::postgres::PostgresDatabase;

pub async fn data_storage(db_url: &str) -> PostgresDatabase {
    let pool = connect_db(db_url).await;
    PostgresDatabase::new(pool)
}

pub async fn connect_db(db_url: &str) -> PgPool {
    match PgPool::connect(db_url).await {
        Ok(pool) => {
            log::info!("database {} successfully connected", db_url);
            pool
        }
        Err(why) => panic!("cannot connect to database: {}", why)
    }
}