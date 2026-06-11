use async_trait::{ async_trait };

use super::super::user::value_objects::{ UserId };
use super::{ Profile };

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &mut Profile) -> Result<(), String>;
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Profile>, String>;
    async fn exists(&self, user_id: UserId) -> Result<bool, String>;
    async fn update(&self, profile: &mut Profile) -> Result<(), String>;
    async fn remove(&self, user_id: UserId) -> Result<(), String>;
}
