use async_trait::{ async_trait };

use super::{ TxContext, SharedDomainError };

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    fn ctx_mut(&mut self) -> &mut dyn TxContext;

    async fn commit(self: Box<Self>) -> Result<(), SharedDomainError>;
    
    async fn rollback(self: Box<Self>) -> Result<(), SharedDomainError>;
}
