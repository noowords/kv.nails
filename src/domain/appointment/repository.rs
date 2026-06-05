use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn get_by_id(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<Option<Appointment>, String>;
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<bool, String>;
    
    async fn update(
        tx: &mut Transaction<'_, MySql>,
        appointment: Appointment
    ) -> Result<(), String>;
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<(), String>;
}
