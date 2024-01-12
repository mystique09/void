use crate::value_object::{id::Id, name::Name};
use chrono::NaiveDate;

pub type UserId = Id<User>;
pub type NickName = Name<User>;

impl NickName {
    pub fn min_len() -> usize {
        1
    }

    pub fn max_len() -> usize {
        24
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    nickname: NickName,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
}

impl User {
    pub fn new(id: UserId, nickname: NickName) -> Self {
        Self {
            id,
            nickname,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn id(&self) -> i64 {
        self.id.to_i64()
    }

    pub fn nickname(&self) -> &str {
        self.nickname.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_user_instance() {
        let user = User::new(UserId::new(1), NickName::new("deeznuts".to_string()));

        assert_eq!(user.id, UserId::new(1));
    }
}