use async_trait::{ async_trait };

use super::super::{
    shared::{ TxContext },
    user::value_objects::{ UserId }
};

use super::{ Master };

#[async_trait]
pub trait MasterRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        master: &mut Master
    ) -> Result<(), String>;
    
    async fn get_by_user_id(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Master>, String>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        master: &mut Master
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), String>;
}
