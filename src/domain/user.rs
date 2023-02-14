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

pub enum UserError {
    UserNotFound,
}

pub trait UserRepository {
    fn get_users() -> Result<Vec<User>, String>;
    fn get_user_by_id(id: i32) -> Option<User>;
    fn get_user_by_username(username: &str) -> Option<User>;
    fn set_user_exp(new_exp: i32) -> Result<i32, String>;
    fn delete_user(id: i32) -> Result<User, String>;
}

pub trait UserUsecase {
    fn get_users() -> Result<Vec<User>, String>;
    fn get_user_by_id(id: i32) -> Option<User>;
    fn get_user_by_username(username: &str) -> Option<User>;
    fn set_user_exp(new_exp: i32) -> Result<i32, String>;
    fn delete_user(id: i32) -> Result<User, String>;
}
