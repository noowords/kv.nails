use std::sync::{ Arc };
use tokio::sync::{ Mutex };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::appointment::{
    Appointment, AppointmentRepository,
    value_objects::{ AppointmentId }
};

use super::{ MySqlAppointmentModel };

pub struct MySqlAppointmentRepository {
    tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>
}

impl MySqlAppointmentRepository {
    pub fn new(tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl AppointmentRepository for MySqlAppointmentRepository {
    async fn create(&self, appointment: Appointment) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        sqlx::query(
            r#"
            INSERT INTO appointments (id, master_id, client_id, date, time, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(appointment.id().value())
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
    
    async fn get_by_id(&self, id: AppointmentId) -> Result<Option<Appointment>, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let appointment_record: Option<MySqlAppointmentModel> = sqlx::query_as(
            r#"
            SELECT id, master_id, client_id, date, time, status
            FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Find appointment error: {}", e))?;

        match appointment_record {
            Some(ar) => Ok(Some(Appointment::try_from(ar)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, id: AppointmentId) -> Result<bool, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM appointments
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Check appointment existence error: {}", e))?;
        
        Ok(row.is_some())
    }
    
    async fn update(&self, appointment: Appointment) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let record = MySqlAppointmentModel::from(&appointment);

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
    
    async fn remove(&self, id: AppointmentId) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Appointment delete error: {}", e))?;
        
        Ok(())
    }
}
