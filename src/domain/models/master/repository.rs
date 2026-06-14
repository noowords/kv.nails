use async_trait::{ async_trait };

use super::super::super::shared::{ UnitOfWork };

use super::super::user::value_objects::{ UserId };

use super::{ Master, MasterModelDomainError };

#[async_trait]
pub trait MasterRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        master: &mut Master
    ) -> Result<(), MasterModelDomainError>;
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<Option<Master>, MasterModelDomainError>;
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<bool, MasterModelDomainError>;
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        master: &mut Master
    ) -> Result<(), MasterModelDomainError>;
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<(), MasterModelDomainError>;
}
