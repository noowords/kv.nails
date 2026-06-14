use serde::{ Deserialize };
use uuid::{ Uuid };

#[derive(Deserialize)]
pub struct CreateMasterCommand {
    pub user_id: Uuid,
    pub schedule: serde_json::Value
}

impl CreateMasterCommand {
    pub fn new(
        user_id: Uuid,
        schedule: serde_json::Value
    ) -> Self {
        Self {
            user_id,
            schedule
        }
    }
}
