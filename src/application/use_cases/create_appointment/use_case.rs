use std::sync::{ Arc };

use crate::domain::models::{
    appointment::{ Appointment, AppointmentRepository }
};

use super::super::super::shared::{ UnitOfWorkFactory };

use super::{ CreateAppointmentCommand, CreateAppointmentUseCaseApplicationError };

pub struct CreateAppointmentUseCase {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    appointment_repository: Arc<dyn AppointmentRepository>
}

impl CreateAppointmentUseCase {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        appointment_repository: Arc<dyn AppointmentRepository>
    ) -> Self {
        Self {
            uow_factory,
            appointment_repository
        }
    }

    pub async fn execute(
        &self,
        cmd: CreateAppointmentCommand
    ) -> Result<(), CreateAppointmentUseCaseApplicationError> {
        let mut uow = self.uow_factory.begin().await?;

        let appointment = Appointment::new(
            None,
            cmd.master_id,
            cmd.client_id,
            cmd.date,
            cmd.time,
            None
        );

        self.appointment_repository.create(&mut *uow, appointment).await?;

        Ok(())
    }
}
