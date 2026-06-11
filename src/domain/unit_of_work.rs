use async_trait::{ async_trait };

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn commit(self: Box<Self>) -> Result<(), String>;
    async fn rollback(self: Box<Self>) -> Result<(), String>;
}