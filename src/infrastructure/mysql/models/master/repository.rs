use async_trait::{ async_trait };
use serde_json;

use crate::domain::shared::{ UnitOfWork };
use crate::domain::models::{
    user::value_objects::{ UserId },
    master::{ Master, MasterRepository }
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
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let schedule_json = serde_json::to_value(master.schedule())
            .map_err(|e| format!("Schedule serializing error: {}", e))?;
            
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
            .map_err(|e| format!("Master insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<Option<Master>, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

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
            .map_err(|e| format!("Master find error: {}", e))?;

        match row {
            Some(row) => Ok(Some(Master::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<bool, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

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
            .map_err(|e| format!("Check master existence error: {}", e))?;
        
        Ok(row.is_some())
    }

    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        master: &mut Master
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

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
            .map_err(|e| format!("Master update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Master delete error: {}", e))?;
        
        Ok(())
    }
}
