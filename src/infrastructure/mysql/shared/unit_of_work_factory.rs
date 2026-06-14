use async_trait::{ async_trait };
use sqlx::mysql::{ MySqlPool };

use crate::domain::shared::{ UnitOfWork };
use crate::application::shared::{ UnitOfWorkFactory, SharedApplicationError };

use super::{ MySqlUnitOfWork };

pub struct MySqlUnitOfWorkFactory {
    pool: MySqlPool
}

impl MySqlUnitOfWorkFactory {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWorkFactory for MySqlUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, SharedApplicationError> {
        let uow = MySqlUnitOfWork::begin(&self.pool).await?;
        
        Ok(Box::new(uow))
    }
}
