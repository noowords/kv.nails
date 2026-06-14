use async_trait::{ async_trait };

use super::super::super::shared::{ UnitOfWork };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn get_by_id(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<Option<User>, String>;
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        user: &mut User
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<(), String>;
}
