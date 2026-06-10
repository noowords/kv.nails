use crate::domain::{
    master::{ MasterRepository },
    appointment::{ Appointment, AppointmentRepository, AppointmentService }
};
use crate::infrastructure::mysql::{
    MySqlUnitOfWork,
    repositories::{ MySqlMasterRepository, MySqlAppointmentRepository }
};

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentUseCase;

impl CreateAppointmentUseCase {
    pub async fn execute(
        uow: &mut MySqlUnitOfWork<'_>,
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

        MySqlAppointmentRepository::create(
            &mut uow.tx(),
            appointment
        ).await
    }
}
