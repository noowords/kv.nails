use std::sync::{ Arc };

use crate::domain::models::{
    master::{ Master, MasterRepository }
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

        let mut master = Master::new(cmd.user_id, cmd.schedule);

        self.master_repository.create(&mut *uow, &mut master).await?;

        Ok(())
    }
}
