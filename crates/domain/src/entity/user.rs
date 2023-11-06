use crate::value_object::{id::Id, name::Name};
use chrono::NaiveDate;

pub type UserId = Id<User>;
pub type NickName = Name<User>;

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
