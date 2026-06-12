use async_trait::{ async_trait };

use super::super::shared::{ TxContext };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<Option<Appointment>, String>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<bool, String>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<(), String>;
}
