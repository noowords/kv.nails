use crate::domain::{
    UnitOfWork,
    appointment::{ Appointment, AppointmentRepository }
};

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentUseCase<'a> {
    appointment_repository: &'a dyn AppointmentRepository,
    uow: Box<dyn UnitOfWork>
}

impl<'a> CreateAppointmentUseCase<'a> {
    pub fn new(
        appointment_repository: &'a dyn AppointmentRepository,
        uow: Box<dyn UnitOfWork>
    ) -> Self {
        Self {
            appointment_repository,
            uow
        }
    }

    pub async fn execute(self, cmd: CreateAppointmentCommand) -> Result<(), String> {
        let appointment = Appointment::new(
            None,
            cmd.master_id,
            cmd.client_id,
            cmd.date,
            cmd.time,
            None
        );

        self.appointment_repository.create(appointment).await?;

        self.uow.commit().await?;

        Ok(())
    }
}
