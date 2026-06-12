use crate::domain::{
    shared::{ UnitOfWork },
    master::{ Master, MasterRepository }
};

use super::{ CreateMasterCommand };

pub struct CreateMasterUseCase {
    master_repository: Box<dyn MasterRepository>,
    uow: Box<dyn UnitOfWork>
}

impl CreateMasterUseCase {
    pub fn new(
        master_repository: Box<dyn MasterRepository>,
        uow: Box<dyn UnitOfWork>
    ) -> Self {
        Self {
            master_repository,
            uow
        }
    }

    pub async fn execute(self, cmd: CreateMasterCommand) -> Result<(), String> {
        let mut master = Master::new(cmd.user_id, cmd.schedule);

        self.master_repository.create(&mut master).await?;

        self.uow.commit().await?;

        Ok(())
    }
}
