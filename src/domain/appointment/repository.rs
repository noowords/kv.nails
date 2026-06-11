use async_trait::{ async_trait };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository {
    async fn create(&self, appointment: Appointment) -> Result<(), String>;
    async fn get_by_id(&self, id: AppointmentId) -> Result<Option<Appointment>, String>;
    async fn exists(&self, id: AppointmentId) -> Result<bool, String>;
    async fn update(&self, appointment: Appointment) -> Result<(), String>;
    async fn remove(&self, id: AppointmentId) -> Result<(), String>;
}
