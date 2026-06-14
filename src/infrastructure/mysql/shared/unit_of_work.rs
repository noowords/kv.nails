use async_trait::{ async_trait };
use sqlx::mysql::{ MySqlPool };

use crate::domain::shared::{ TxContext, UnitOfWork };

use super::{ MySqlTxContext };

pub struct MySqlUnitOfWork {
    pub ctx: MySqlTxContext
}

impl MySqlUnitOfWork {
    pub async fn begin(pool: &MySqlPool) -> Result<Self, String> {
        let tx = pool.begin().await
            .map_err(|e| format!("Transaction begin error: {}", e))?;
        let ctx = MySqlTxContext::new(tx);
        
        Ok(Self { ctx })
    }
}

#[async_trait]
impl UnitOfWork for MySqlUnitOfWork {
    fn ctx_mut(&mut self) -> &mut dyn TxContext {
        &mut self.ctx
    }

    async fn commit(self: Box<Self>) -> Result<(), String> {
        self.ctx.tx.commit().await
            .map_err(|e| format!("Commit error: {}", e))
    }

    async fn rollback(self: Box<Self>) -> Result<(), String> {
        self.ctx.tx.rollback().await
            .map_err(|e| format!("Rollback error: {}", e))
    }
}
