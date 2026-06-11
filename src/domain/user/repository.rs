use async_trait::{ async_trait };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &mut User) -> Result<(), String>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, String>;
    async fn exists(&self, id: UserId) -> Result<bool, String>;
    async fn update(&self, user: &mut User) -> Result<(), String>;
    async fn remove(&self, id: UserId) -> Result<(), String>;
}
