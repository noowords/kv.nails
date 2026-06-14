use async_trait::{ async_trait };

use crate::domain::shared::{ UnitOfWork };
use crate::domain::models::user::{
    User, UserRepository,
    value_objects::{ UserId }
};

use super::super::super::shared::{ MySqlTxContext };

use super::{ MySqlUserRow };

pub struct MySqlUserRepository;

impl MySqlUserRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        user: &mut User
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        sqlx::query(
            r#"
            INSERT INTO users (id, phone, role, created_at)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(user.id().value())
            .bind(user.phone().as_ref().map(|p| p.value().clone()))
            .bind(user.role().as_str())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("User insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_id(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<Option<User>, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row: Option<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT id, role, phone
            FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Find user error: {}", e))?;

        match row {
            Some(row) => Ok(Some(User::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<bool, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM users
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Check user existence error: {}", e))?;
        
        Ok(row.is_some())
    }

    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        user: &mut User
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row = MySqlUserRow::from(&*user);

        sqlx::query(
            r#"
            UPDATE users
            SET phone = ?, role = ?
            WHERE id = ?
            "#
        )
            .bind(row.phone())
            .bind(row.role())
            .bind(row.id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("User update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        id: UserId
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("User delete error: {}", e))?;
        
        Ok(())
    }
}
