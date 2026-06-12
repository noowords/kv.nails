use std::sync::{ Arc };
use tokio::sync::{ Mutex };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };
use serde_json;

use crate::domain::{
    user::value_objects::{ UserId },
    master::{ Master, MasterRepository }
};

use super::{ MySqlMasterModel };

pub struct MySqlMasterRepository {
    tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>
}

impl MySqlMasterRepository {
    pub fn new(tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl MasterRepository for MySqlMasterRepository {
    async fn create(&self, master: &mut Master) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

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
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Master insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Master>, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let master_record: Option<MySqlMasterModel> = sqlx::query_as(
            r#"
            SELECT user_id, schedule
            FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Master find error: {}", e))?;

        match master_record {
            Some(mr) => Ok(Some(Master::try_from(mr)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, user_id: UserId) -> Result<bool, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM masters
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Check master existence error: {}", e))?;
        
        Ok(row.is_some())
    }

    async fn update(&self, master: &mut Master) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let record = MySqlMasterModel::from(&*master);

        sqlx::query(
            r#"
            UPDATE masters SET schedule = ?
            WHERE user_id = ?
            "#
        )
            .bind(record.schedule())
            .bind(record.user_id())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Master update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(&self, user_id: UserId) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Master delete error: {}", e))?;
        
        Ok(())
    }
}
