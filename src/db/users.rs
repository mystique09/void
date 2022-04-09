use sqlx::postgres::PgPool;

pub async fn create_user(db: &PgPool, user_id: i64) {
    let new_user = sqlx::query!(
        r#"insert into "user" (dc_id) values($1) returning user_id"#,
        user_id,
    )
    .fetch_one(db)
    .await;
}

pub async fn get_user(db: &PgPool) {}
