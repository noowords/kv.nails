use async_trait::{ async_trait };

use crate::domain::shared::{ UnitOfWork };

use super::{ SharedApplicationError };

#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, SharedApplicationError>;
}
