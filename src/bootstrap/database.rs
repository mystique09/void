use sqlx::{PgPool, Pool, Postgres};
use tracing::info;

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(env: &super::env::Env) -> Self {
        let db_url = env.get_db_url();
        let pool = PgPool::connect(db_url).await.unwrap();

        if let Err(why) = sqlx::migrate!().run(&pool).await {
            info!("Error migrating {:?}", why);
        }

        Self { pool }
    }
}
