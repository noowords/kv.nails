use async_trait::{ async_trait };

use super::super::super::shared::{ UnitOfWork };

use super::{
    Appointment, AppointmentModelDomainError,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), AppointmentModelDomainError>;
    
    async fn get_by_id(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<Option<Appointment>, AppointmentModelDomainError>;
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<bool, AppointmentModelDomainError>;
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), AppointmentModelDomainError>;
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<(), AppointmentModelDomainError>;
}
