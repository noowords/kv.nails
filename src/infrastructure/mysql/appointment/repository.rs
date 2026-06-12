use async_trait::{ async_trait };

use crate::domain::{
    shared::{ TxContext },
    appointment::{
        Appointment, AppointmentRepository,
        value_objects::{ AppointmentId }
    }
};

use super::super::shared::{ MySqlTxContext };

use super::{ MySqlAppointmentModel };

pub struct MySqlAppointmentRepository;

impl MySqlAppointmentRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AppointmentRepository for MySqlAppointmentRepository {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        appointment: Appointment
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

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
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Appointment insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<Option<Appointment>, String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        let model: Option<MySqlAppointmentModel> = sqlx::query_as(
            r#"
            SELECT id, master_id, client_id, date, time, status
            FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Find appointment error: {}", e))?;

        match model {
            Some(model) => Ok(Some(Appointment::try_from(model)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<bool, String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM appointments
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Check appointment existence error: {}", e))?;
        
        Ok(row.is_some())
    }
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        appointment: Appointment
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        let model = MySqlAppointmentModel::from(&appointment);

        sqlx::query(
            r#"
            UPDATE appointments
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
        )
            .bind(model.status())
            .bind(model.id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Appointment update error: {}", e))?;
        
        Ok(())
    }
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Appointment delete error: {}", e))?;
        
        Ok(())
    }
}
