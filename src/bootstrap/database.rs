use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(env: &super::env::Env) -> Self {
        let db_url = env.get_db_url();
        let pool = PgPool::connect(db_url).await.unwrap();

        if let Err(value) = sqlx::migrate!().run(&pool).await {
            println!("Version mismatch {:?}", value);
        }

        Self { pool }
    }
}
