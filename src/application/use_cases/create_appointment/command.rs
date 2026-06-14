use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct CreateAppointmentCommand {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl CreateAppointmentCommand {
    pub fn new(
        master_id: Uuid,
        client_id: Uuid,
        date: NaiveDate,
        time: NaiveTime
    ) -> Self {
        Self {
            master_id,
            client_id,
            date,
            time
        }
    }
}
