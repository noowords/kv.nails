use async_trait::{ async_trait };
use sqlx::mysql::{ MySqlPool };

use crate::domain::shared::{ TxContext, UnitOfWork, SharedDomainError };

use super::{ MySqlTxContext, SharedMySqlInfrastructureError };

pub struct MySqlUnitOfWork {
    pub ctx: MySqlTxContext
}

impl MySqlUnitOfWork {
    pub async fn begin(pool: &MySqlPool) -> Result<Self, SharedDomainError> {
        let tx = pool.begin().await
            .map_err(SharedMySqlInfrastructureError::from_sqlx)
            .map_err(|e| SharedDomainError::TransactionBeginFailed(e.to_string()))?;
        
        let ctx = MySqlTxContext::new(tx);
        
        Ok(Self { ctx })
    }
}

#[async_trait]
impl UnitOfWork for MySqlUnitOfWork {
    fn ctx_mut(&mut self) -> &mut dyn TxContext {
        &mut self.ctx
    }

    async fn commit(self: Box<Self>) -> Result<(), SharedDomainError> {
        self.ctx.tx.commit().await
            .map_err(SharedMySqlInfrastructureError::from_sqlx)
            .map_err(|e| SharedDomainError::TransactionCommitFailed(e.to_string()))
    }

    async fn rollback(self: Box<Self>) -> Result<(), SharedDomainError> {
        self.ctx.tx.rollback().await
            .map_err(SharedMySqlInfrastructureError::from_sqlx)
            .map_err(|e| SharedDomainError::TransactionRollbackFailed(e.to_string()))
    }
}
