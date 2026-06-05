use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn get_by_id(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<Option<User>, String>;
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<bool, String>;

    async fn update(
        tx: &mut Transaction<'_, MySql>,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<(), String>;
}
