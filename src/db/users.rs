use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Default)]
pub struct User {
    pub user_id: i64,
    pub user_name: String,
    pub dc_id: i64,
    pub user_balance: i64,
    pub user_rank: i64,
    pub user_exp: i64,
}

pub trait TUser {
    fn get_id(&self) -> i64;
    fn get_dc_id(&self) -> i64;
    fn get_balance(&self) -> i64;
    fn get_rank(&self) -> i64;
    fn get_exp(&self) -> i64;
}

impl TUser for User {
    fn get_id(&self) -> i64 {
        self.user_id
    }

    fn get_dc_id(&self) -> i64 {
        self.dc_id
    }

    fn get_balance(&self) -> i64 {
        self.user_balance
    }

    fn get_rank(&self) -> i64 {
        self.user_rank
    }

    fn get_exp(&self) -> i64 {
        self.user_exp
    }
}

pub async fn get_user(pool: &PgPool, id: i64) -> anyhow::Result<User, sqlx::Error> {
    let query = sqlx::query_as!(
        User,
        r#"
    SELECT user_id, dc_id, user_balance, user_rank, user_exp, user_name
    FROM "user"
    WHERE dc_id = $1
    "#,
        id
    )
    .fetch_one(pool)
    .await;

    query
}

pub async fn new_user(pool: &PgPool, id: i64, name: String) -> anyhow::Result<i64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO "user"(dc_id, user_name)
        VALUES($1, $2)
        RETURNING dc_id
        "#,
        id,
        name
    )
    .fetch_one(pool)
    .await;

    Ok(query.unwrap().dc_id)
}

pub async fn update_user(pool: &PgPool, user: &User) -> anyhow::Result<bool> {
    let query = sqlx::query!(
        r#"
    UPDATE "user"
    SET user_exp = (user_exp + 1) % 20
    ,user_rank = CASE WHEN user_exp = 19
    THEN user_rank + 1 ELSE user_rank END
    WHERE dc_id = $1
    "#,
        user.dc_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(query > 0)
}

pub async fn delete_user(pool: &PgPool, id: i64) -> anyhow::Result<bool> {
    let query = sqlx::query!(
        r#"
    DELETE FROM "user"
    WHERE dc_id = $1
    "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(query > 0)
}
