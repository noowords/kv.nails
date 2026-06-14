use std::sync::{ Arc };

use crate::domain::models::{
    user::value_objects::{ UserId },
    master::{ Master, MasterRepository, MasterModelDomainError }
};

use super::super::super::shared::{ UnitOfWorkFactory };

use super::{ CreateMasterCommand, CreateMasterUseCaseApplicationError };

pub struct CreateMasterUseCase {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    master_repository: Arc<dyn MasterRepository>
}

impl CreateMasterUseCase {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        master_repository: Arc<dyn MasterRepository>
    ) -> Self {
        Self {
            uow_factory,
            master_repository
        }
    }

    pub async fn execute(
        &self,
        cmd: CreateMasterCommand
    ) -> Result<(), CreateMasterUseCaseApplicationError> {
        let mut uow = self.uow_factory.begin().await?;

        let mut master = Master::new(
            UserId::from(cmd.user_id),
            serde_json::from_value(cmd.schedule)
                .map_err(|e| MasterModelDomainError::CorruptedSchedule(e.to_string()))?
        );

        self.master_repository.create(&mut *uow, &mut master).await?;

        Ok(())
    }
}
