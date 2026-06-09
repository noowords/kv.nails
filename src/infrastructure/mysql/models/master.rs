use uuid::{ Uuid };
use sqlx::{ FromRow };

use crate::domain::{
    user::value_objects::{ UserId },
    master::{
        Master,
        value_objects::{ Schedule }
    }
};

#[derive(FromRow)]
pub struct MasterRecord {
    user_id: Uuid,
    schedule: serde_json::Value
}

impl MasterRecord {
    pub fn new(
        user_id: Uuid,
        schedule: serde_json::Value
    ) -> Self {
        Self {
            user_id,
            schedule
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    
    pub fn schedule(&self) -> &serde_json::Value {
        &self.schedule
    }
}

impl TryFrom<MasterRecord> for Master {
    type Error = String;
    
    fn try_from(record: MasterRecord) -> Result<Self, Self::Error> {
        let schedule: Schedule = serde_json::from_value(record.schedule)
            .map_err(|e| format!("Schedule deserialization error: {}", e))?;
        
        Ok(Self::new(
            UserId::from(record.user_id),
            schedule
        ))
    }
}

impl From<&Master> for MasterRecord {
    fn from(master: &Master) -> Self {
        Self {
            user_id: master.user_id().value(),
            schedule: serde_json::to_value(master.schedule())
                .expect("Schedule serialization failed")
        }
    }
}
