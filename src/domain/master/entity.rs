use super::super::user::value_objects::{ UserId };
use super::value_objects::{ Schedule };

#[derive(Clone, Eq, PartialEq)]
pub struct Master {
    user_id: UserId,
    schedule: Schedule
}

impl Master {
    pub fn new(user_id: UserId, schedule: Schedule) -> Self {
        Self {
            user_id,
            schedule
        }
    }

    pub fn from_record(user_id: UserId, schedule: Schedule) -> Self {
        Self {
            user_id,
            schedule
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
}
