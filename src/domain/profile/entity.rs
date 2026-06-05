use super::super::user::value_objects::{ UserId };

#[derive(Clone, Eq, PartialEq)]
pub struct Profile {
    user_id: UserId
}

impl Profile {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id
        }
    }

    pub fn from_record(user_id: UserId) -> Self {
        Self {
            user_id
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}
