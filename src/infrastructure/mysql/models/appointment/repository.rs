use async_trait::{ async_trait };

use crate::domain::shared::{ UnitOfWork };
use crate::domain::models::appointment::{
    Appointment, AppointmentRepository, AppointmentModelDomainError,
    value_objects::{ AppointmentId }
};

use super::super::super::shared::{ MySqlTxContext };

use super::{ MySqlAppointmentRow };

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
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), AppointmentModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| AppointmentModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

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
            .map_err(|e| AppointmentModelDomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_id(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<Option<Appointment>, AppointmentModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| AppointmentModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let row: Option<MySqlAppointmentRow> = sqlx::query_as(
            r#"
            SELECT id, master_id, client_id, date, time, status
            FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| AppointmentModelDomainError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Appointment::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<bool, AppointmentModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| AppointmentModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

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
            .map_err(|e| AppointmentModelDomainError::DatabaseError(e.to_string()))?;
        
        Ok(row.is_some())
    }
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        appointment: Appointment
    ) -> Result<(), AppointmentModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| AppointmentModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let row = MySqlAppointmentRow::from(&appointment);

        sqlx::query(
            r#"
            UPDATE appointments
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
        )
            .bind(row.status())
            .bind(row.id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| AppointmentModelDomainError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        id: AppointmentId
    ) -> Result<(), AppointmentModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| AppointmentModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| AppointmentModelDomainError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}
