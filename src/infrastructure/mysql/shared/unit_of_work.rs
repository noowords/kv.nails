use std::sync::{ Arc };
use tokio::sync::{ Mutex };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::shared::{ UnitOfWork };

pub struct MySqlUnitOfWork {
    tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>
}

impl MySqlUnitOfWork {
    pub fn new(tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UnitOfWork for MySqlUnitOfWork {
    async fn commit(self: Box<Self>) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        
        if let Some(tx) = tx_guard.take() {
            tx.commit().await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        
        if let Some(tx) = tx_guard.take() {
            tx.rollback().await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
