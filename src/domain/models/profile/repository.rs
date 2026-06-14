use async_trait::{ async_trait };

use super::super::super::shared::{ UnitOfWork };

use super::super::user::value_objects::{ UserId };

use super::{ Profile, ProfileModelDomainError };

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        profile: &mut Profile
    ) -> Result<(), ProfileModelDomainError>;
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<Option<Profile>, ProfileModelDomainError>;
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<bool, ProfileModelDomainError>;
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        profile: &mut Profile
    ) -> Result<(), ProfileModelDomainError>;
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<(), ProfileModelDomainError>;
}
