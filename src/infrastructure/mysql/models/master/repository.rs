use async_trait::{ async_trait };
use serde_json;

use crate::domain::shared::{ UnitOfWork };
use crate::domain::models::{
    user::value_objects::{ UserId },
    master::{ Master, MasterRepository, MasterModelDomainError }
};

use super::super::super::shared::{ MySqlTxContext };

use super::{ MySqlMasterRow };

pub struct MySqlMasterRepository;

impl MySqlMasterRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MasterRepository for MySqlMasterRepository {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        master: &mut Master
    ) -> Result<(), MasterModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| MasterModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let schedule_json = serde_json::to_value(master.schedule())
            .map_err(|e| MasterModelDomainError::CorruptedSchedule(e.to_string()))?;
            
        sqlx::query(
            r#"
            INSERT INTO masters (user_id, schedule)
            VALUES (?, ?)
            "#
        )
            .bind(master.user_id().value())
            .bind(&schedule_json)
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| MasterModelDomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<Option<Master>, MasterModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| MasterModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let row: Option<MySqlMasterRow> = sqlx::query_as(
            r#"
            SELECT user_id, schedule
            FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| MasterModelDomainError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Master::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<bool, MasterModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| MasterModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM masters
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| MasterModelDomainError::DatabaseError(e.to_string()))?;
        
        Ok(row.is_some())
    }

    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        master: &mut Master
    ) -> Result<(), MasterModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| MasterModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        let row = MySqlMasterRow::from(&*master);

        sqlx::query(
            r#"
            UPDATE masters SET schedule = ?
            WHERE user_id = ?
            "#
        )
            .bind(row.schedule())
            .bind(row.user_id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| MasterModelDomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<(), MasterModelDomainError> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| MasterModelDomainError::DatabaseError("Invalid UnitOfWork context".to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| MasterModelDomainError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}
