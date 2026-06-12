use async_trait::{ async_trait };

use super::super::super::shared::{ TxContext };

use super::super::user::value_objects::{ UserId };

use super::{ Profile };

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        profile: &mut Profile
    ) -> Result<(), String>;
    
    async fn get_by_user_id(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Profile>, String>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        profile: &mut Profile
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), String>;
}
