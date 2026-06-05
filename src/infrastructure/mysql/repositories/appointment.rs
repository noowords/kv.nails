use chrono::{ NaiveDate };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::appointment::{
    Appointment, AppointmentRepository,
    value_objects::{ AppointmentId }
};

use super::super::models::{ AppointmentRecord };

pub struct MySqlAppointmentRepository;

#[async_trait]
impl AppointmentRepository for MySqlAppointmentRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        appointment: Appointment
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO appointments (master_id, client_id, date, time, status, created_at)
            VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(appointment.master_id().value())
            .bind(appointment.client_id().value())
            .bind(appointment.date())
            .bind(appointment.time().format("%H:%M:%S").to_string())
            .bind(appointment.status().as_str())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Appointment insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_id(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<Option<Appointment>, String> {
        unimplemented!()
    }
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<bool, String> {
        unimplemented!()
    }
    
    async fn update(
        tx: &mut Transaction<'_, MySql>,
        appointment: Appointment
    ) -> Result<(), String> {
        let record = AppointmentRecord::from(&appointment);

        sqlx::query(
            r#"
            UPDATE appointments
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
        )
            .bind(record.status())
            .bind(record.id())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Appointment update error: {}", e))?;
        
        Ok(())
    }
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        id: AppointmentId
    ) -> Result<(), String> {
        unimplemented!()
    }
}
