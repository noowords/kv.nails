use uuid::{ Uuid };
use sqlx::{ FromRow };

use crate::domain::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

#[derive(FromRow)]
pub struct ProfileRecord {
    user_id: Uuid,
    first_name: String,
    last_name: String,
    avatar_url: Option<String>,
    bio: Option<String>
}

impl ProfileRecord {
    pub fn new(
        user_id: Uuid,
        first_name: String,
        last_name: String,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self {
            user_id,
            first_name,
            last_name,
            avatar_url,
            bio
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
}

impl TryFrom<ProfileRecord> for Profile {
    type Error = String;
    
    fn try_from(record: ProfileRecord) -> Result<Self, Self::Error> {
        Ok(Self::new(
            UserId::from(record.user_id),
            record.first_name,
            record.last_name,
            record.avatar_url,
            record.bio
        ))
    }
}

impl From<&Profile> for ProfileRecord {
    fn from(profile: &Profile) -> Self {
        Self {
            user_id: profile.user_id().value(),
            first_name: profile.first_name().to_string(),
            last_name: profile.last_name().to_string(),
            avatar_url: profile.avatar_url().clone(),
            bio: profile.bio().clone()
        }
    }
}
