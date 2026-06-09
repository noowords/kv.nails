use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use super::super::user::value_objects::{ UserId };
use super::{ Master };

#[async_trait]
pub trait MasterRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        master: &mut Master
    ) -> Result<(), String>;
    
    async fn get_by_user_id(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<Option<Master>, String>;
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<bool, String>;
    
    async fn update(
        tx: &mut Transaction<'_, MySql>,
        master: &mut Master
    ) -> Result<(), String>;
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<(), String>;
}
