use std::sync::{ Arc };

use crate::domain::{
    shared::{ TxContext },
    master::{ Master, MasterRepository }
};

use super::{ CreateMasterCommand };

pub struct CreateMasterUseCase {
    master_repository: Arc<dyn MasterRepository>
}

impl CreateMasterUseCase {
    pub fn new(
        master_repository: Arc<dyn MasterRepository>
    ) -> Self {
        Self {
            master_repository
        }
    }

    pub async fn execute(
        &self,
        ctx: &mut dyn TxContext,
        cmd: CreateMasterCommand
    ) -> Result<(), String> {
        let mut master = Master::new(cmd.user_id, cmd.schedule);

        self.master_repository.create(ctx, &mut master).await?;

        Ok(())
    }
}
