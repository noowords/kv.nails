use chrono::{ NaiveDate, NaiveTime };

use super::super::user::value_objects::{ UserId };
use super::value_objects::{ AppointmentId, AppointmentStatus };

#[derive(Clone)]
pub struct Appointment {
    id: AppointmentId,
    master_id: UserId,
    client_id: UserId,
    date: NaiveDate,
    time: NaiveTime,
    status: AppointmentStatus
}

impl Appointment {
    pub fn new(
        id: Option<AppointmentId>,
        master_id: UserId,
        client_id: UserId,
        date: NaiveDate,
        time: NaiveTime,
        status: Option<AppointmentStatus>
    ) -> Self {
        Self {
            id: id.unwrap_or(AppointmentId::new()),
            master_id,
            client_id,
            date,
            time,
            status: status.unwrap_or(AppointmentStatus::Pending)
        }
    }

    pub fn id(&self) -> AppointmentId {
        self.id
    }

    pub fn master_id(&self) -> UserId {
        self.master_id
    }

    pub fn client_id(&self) -> UserId {
        self.client_id
    }

    pub fn time(&self) -> NaiveTime {
        self.time
    }
    
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    
    pub fn status(&self) -> &AppointmentStatus {
        &self.status
    }

    pub fn cancel(&mut self) {
        self.status = AppointmentStatus::Cancelled;
    }

    pub fn complete(&mut self) {
        self.status = AppointmentStatus::Completed;
    }
}
