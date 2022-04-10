use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Default)]
pub struct User {
    pub user_id: i64,
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

pub async fn get_user(_pool: &PgPool, _id: i64) -> anyhow::Result<User, sqlx::Error> {
    let query = sqlx::query_as!(
        User,
        r#"
    SELECT user_id, dc_id, user_balance, user_rank, user_exp
    FROM "user"
    WHERE dc_id = $1
    "#,
        _id
    )
    .fetch_one(_pool)
    .await;

    query
}

pub async fn new_user(_pool: &PgPool, _id: i64) -> anyhow::Result<i64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO "user"(dc_id)
        VALUES($1)
        RETURNING dc_id
        "#,
        _id
    )
    .fetch_one(_pool)
    .await;

    Ok(query.unwrap().dc_id)
}

pub async fn update_user(_pool: &PgPool, user: &User) -> anyhow::Result<bool> {
    let new_exp = (user.user_exp + 1) % 20;
    let new_rank = {
        if new_exp == 0 {
            user.user_rank + 1
        } else {
            user.user_rank
        }
    };

    let query = sqlx::query!(
        r#"
    UPDATE "user"
    SET user_exp = $1, user_rank = $2
    WHERE dc_id = $3
    "#,
        new_exp,
        new_rank,
        user.dc_id
    )
    .execute(_pool)
    .await?
    .rows_affected();

    Ok(query > 0)
}

pub async fn delete_user(_pool: &PgPool, _id: i64) -> anyhow::Result<()> {
    Ok(())
}
