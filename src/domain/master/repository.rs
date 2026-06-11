use async_trait::{ async_trait };

use super::super::user::value_objects::{ UserId };
use super::{ Master };

#[async_trait]
pub trait MasterRepository {
    async fn create(&self, master: &mut Master) -> Result<(), String>;
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Master>, String>;
    async fn exists(&self, user_id: UserId) -> Result<bool, String>;
    async fn update(&self, master: &mut Master) -> Result<(), String>;
    async fn remove(&self, user_id: UserId) -> Result<(), String>;
}
