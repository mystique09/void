use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub uid: String,
    pub wallet: i64,
    pub bank: i64,
    pub diamond: i64,
    pub rank: i64,
    pub exp: i32,
    pub guild_id: String,
}

pub async fn get_user(pool: &PgPool, id: &str) -> anyhow::Result<User, sqlx::Error> {
    let query = sqlx::query_as!(
        User,
        r#"
    SELECT id, uid, username, wallet,  bank, diamond, rank, exp, guild_id
    FROM "profile"
    WHERE uid = $1
    "#,
        id
    )
    .fetch_one(pool)
    .await;

    query
}

pub async fn new_user(
    pool: &PgPool,
    id: &str,
    guild: &str,
    name: &str,
) -> anyhow::Result<String, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO "profile"(uid, username, guild_id)
        VALUES($1, $2, $3)
        RETURNING uid
        "#,
        id,
        name,
        guild
    )
    .fetch_one(pool)
    .await;

    Ok(query.unwrap().uid)
}

pub async fn set_exp(pool: &PgPool, user: &User) -> anyhow::Result<bool> {
    let query = sqlx::query!(
        r#"
    UPDATE "profile"
    SET exp = (exp + 1) % 20
    ,rank = CASE WHEN exp = 19
    THEN rank + 1 ELSE rank END
    WHERE uid = $1
    "#,
        user.uid
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(query > 0)
}

pub async fn delete_user(pool: &PgPool, id: &str) -> anyhow::Result<bool> {
    let query = sqlx::query!(
        r#"
    DELETE FROM "profile"
    WHERE uid = $1
    "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(query > 0)
}
