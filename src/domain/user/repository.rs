use async_trait::{ async_trait };

use super::super::shared::{ TxContext };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<Option<User>, String>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<(), String>;
}
