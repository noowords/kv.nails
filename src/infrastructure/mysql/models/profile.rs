use uuid::{ Uuid };
use sqlx::{ FromRow };

use crate::domain::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

#[derive(FromRow)]
pub struct ProfileRecord {
    user_id: Uuid
}

impl ProfileRecord {
    pub fn new(
        user_id: Uuid
    ) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

impl TryFrom<ProfileRecord> for Profile {
    type Error = String;
    
    fn try_from(record: ProfileRecord) -> Result<Self, Self::Error> {
        Ok(Self::from_record(
            UserId::from(record.user_id)
        ))
    }
}

impl From<&Profile> for ProfileRecord {
    fn from(profile: &Profile) -> Self {
        Self {
            user_id: profile.user_id().value()
        }
    }
}
