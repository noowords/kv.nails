use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use sqlx::{ FromRow };

use crate::domain::models::{
    user::value_objects::{ UserId },
    appointment::{
        Appointment,
        value_objects::{ AppointmentId, AppointmentStatus }
    }
};

#[derive(FromRow)]
pub struct MySqlAppointmentRow {
    id: Uuid,
    master_id: Uuid,
    client_id: Uuid,
    date: NaiveDate,
    time: NaiveTime,
    status: String
}

impl MySqlAppointmentRow {
    pub fn new(
        id: Uuid,
        master_id: Uuid,
        client_id: Uuid,
        date: NaiveDate,
        time: NaiveTime,
        status: String
    ) -> Self {
        Self {
            id,
            master_id,
            client_id,
            date,
            time,
            status
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    
    pub fn master_id(&self) -> Uuid {
        self.master_id
    }
    
    pub fn client_id(&self) -> Uuid {
        self.client_id
    }
    
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    
    pub fn time(&self) -> NaiveTime {
        self.time
    }
    
    pub fn status(&self) -> &str {
        &self.status
    }
}

impl TryFrom<MySqlAppointmentRow> for Appointment {
    type Error = String;
    
    fn try_from(record: MySqlAppointmentRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            Some(AppointmentId::from(record.id)),
            UserId::from(record.master_id),
            UserId::from(record.client_id),
            record.date,
            record.time,
            Some(AppointmentStatus::from_str(&record.status)
                .ok_or_else(|| format!("Unknown status: {}", record.status))?)
        ))
    }
}

impl From<&Appointment> for MySqlAppointmentRow {
    fn from(appointment: &Appointment) -> Self {
        Self {
            id: appointment.id().value(),
            master_id: appointment.master_id().value(),
            client_id: appointment.client_id().value(),
            date: appointment.date(),
            time: appointment.time(),
            status: appointment.status().as_str().to_string()
        }
    }
}
