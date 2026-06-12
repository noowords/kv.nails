use std::sync::{ Arc };

use crate::domain::shared::{ TxContext };
use crate::domain::models::{
    appointment::{ Appointment, AppointmentRepository }
};

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentUseCase {
    appointment_repository: Arc<dyn AppointmentRepository>
}

impl CreateAppointmentUseCase {
    pub fn new(
        appointment_repository: Arc<dyn AppointmentRepository>
    ) -> Self {
        Self {
            appointment_repository
        }
    }

    pub async fn execute(
        &self,
        ctx: &mut dyn TxContext,
        cmd: CreateAppointmentCommand
    ) -> Result<(), String> {
        let appointment = Appointment::new(
            None,
            cmd.master_id,
            cmd.client_id,
            cmd.date,
            cmd.time,
            None
        );

        self.appointment_repository.create(ctx, appointment).await?;

        Ok(())
    }
}
