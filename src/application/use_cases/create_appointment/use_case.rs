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
        // Добавить получение уже существующих записей у мастера
        // Добавить проверку на роль пользователя для записи
        
        let master = MySqlMasterRepository::get_by_user_id(
            &mut uow.tx(),
            cmd.master_id
        ).await?.unwrap();
        
        let schedule = master.schedule();
        
        if !AppointmentService::can_book(schedule, &vec![], &cmd.date, &cmd.time) {
            return Err("Cannot book this slot".to_string());
        }
        
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
        ).await?;

        Ok(())
    }
}
