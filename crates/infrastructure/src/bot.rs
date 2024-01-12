use std::sync::{Arc, RwLock};

use sqlx::{PgPool, Pool, Postgres};

use void_db::CloudDatabase;

pub async fn run(db_url: &str, port: u16) {
    let pool: PgPool = Pool::<Postgres>::connect(db_url).await.unwrap();
    let db = CloudDatabase::new(pool);
    let safe_db = RwLock::new(Arc::new(db));

    void_discord_bot::run(safe_db, port).await;
}