use async_trait::{ async_trait };

use super::super::super::shared::{ UnitOfWork };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn get_by_id(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<Option<Appointment>, String>;
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<(), String>;
}
