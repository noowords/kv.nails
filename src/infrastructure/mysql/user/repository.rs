use std::sync::{ Arc };
use tokio::sync::{ Mutex };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::user::{
    User, UserRepository,
    value_objects::{ UserId }
};

use super::{ MySqlUserModel };

pub struct MySqlUserRepository {
    tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>
}

impl MySqlUserRepository {
    pub fn new(tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(&self, user: &mut User) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        sqlx::query(
            r#"
            INSERT INTO users (id, phone, role, created_at)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(user.id().value())
            .bind(user.phone().as_ref().map(|p| p.value().clone()))
            .bind(user.role().as_str())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("User insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let user_record: Option<MySqlUserModel> = sqlx::query_as(
            r#"
            SELECT id, role, phone
            FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Find user error: {}", e))?;

        match user_record {
            Some(ur) => Ok(Some(User::try_from(ur)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, id: UserId) -> Result<bool, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM users
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Check user existence error: {}", e))?;
        
        Ok(row.is_some())
    }

    async fn update(&self, user: &mut User) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let user_record = MySqlUserModel::from(&*user);

        sqlx::query(
            r#"
            UPDATE users
            SET phone = ?, role = ?
            WHERE id = ?
            "#
        )
            .bind(user_record.phone())
            .bind(user_record.role())
            .bind(user_record.id())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("User update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(&self, id: UserId) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("User delete error: {}", e))?;
        
        Ok(())
    }
}
