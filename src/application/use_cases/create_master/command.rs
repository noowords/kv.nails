use crate::domain::{
    user::value_objects::{ UserId },
    master::value_objects::{ Schedule }
};

pub struct CreateMasterCommand {
    pub user_id: UserId,
    pub schedule: Schedule
}

impl CreateMasterCommand {
    pub fn new(
        user_id: UserId,
        schedule: Schedule
    ) -> Self {
        Self {
            user_id,
            schedule
        }
    }
}
