use crate::domain::master::{
    Master, MasterRepository
};
use crate::infrastructure::mysql::{
    MySqlUnitOfWork,
    repositories::{ MySqlMasterRepository }
};

use super::{ CreateMasterCommand };

pub struct CreateMasterUseCase;

impl CreateMasterUseCase {
    pub async fn execute(
        uow: &mut MySqlUnitOfWork<'_>,
        cmd: CreateMasterCommand
    ) -> Result<(), String> {
        let mut master = Master::new(cmd.user_id, cmd.schedule);

        MySqlMasterRepository::create(
            &mut uow.tx(),
            &mut master
        ).await;

        Ok(())
    }
}
