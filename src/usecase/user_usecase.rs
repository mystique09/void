use crate::domain;

pub struct UserUsecase {}

impl domain::user::UserUsecase for UserUsecase {
    fn get_users() -> Result<Vec<domain::user::User>, String> {
        todo!()
    }

    fn get_user_by_id(_id: i32) -> Option<domain::user::User> {
        todo!()
    }

    fn get_user_by_username(_username: &str) -> Option<domain::user::User> {
        todo!()
    }

    fn set_user_exp(_new_exp: i32) -> Result<i32, String> {
        todo!()
    }

    fn delete_user(_id: i32) -> Result<domain::user::User, String> {
        todo!()
    }
}
