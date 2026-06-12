use crate::domain::{
    shared::{ UnitOfWork },
    appointment::{ Appointment, AppointmentRepository }
};

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentUseCase {
    appointment_repository: Box<dyn AppointmentRepository>,
    uow: Box<dyn UnitOfWork>
}

impl CreateAppointmentUseCase {
    pub fn new(
        appointment_repository: Box<dyn AppointmentRepository>,
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
