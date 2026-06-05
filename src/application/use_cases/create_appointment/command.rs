use chrono::{ NaiveDate, NaiveTime };
use crate::domain::user::value_objects::{ UserId };

pub struct CreateAppointmentCommand {
    pub master_id: UserId,
    pub client_id: UserId,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl CreateAppointmentCommand {
    pub fn new(
        master_id: UserId,
        client_id: UserId,
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
