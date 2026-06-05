use chrono::{ Utc, NaiveDate, NaiveTime, Datelike };

use super::super::master::value_objects::{ Schedule };
use super::{
    Appointment,
    value_objects::{ AppointmentId, AppointmentStatus }
};

pub struct AppointmentService;

impl AppointmentService {
    pub fn can_book(
        schedule: &Schedule,
        existing_appointments: &Vec<Appointment>,
        date: &NaiveDate,
        time: &NaiveTime
    ) -> bool {
        let weekday = date.weekday();
    
        let Some(work_day) = schedule.work_day(weekday) else {
            return false;
        };

        if !work_day.slots().contains(time) {
            return false;
        }

        let today = Utc::now().date_naive();
        if *date < today {
            return false;
        }

        if *date == today {
            let now_time = Utc::now().time();
            if *time <= now_time {
                return false;
            }
        }

        let is_slot_taken = existing_appointments.iter().any(|a| 
            a.date() == *date && 
            a.time() == *time && 
            matches!(a.status(), AppointmentStatus::Pending | AppointmentStatus::Confirmed)
        );

        if is_slot_taken {
            return false;
        }

        true
    }

    pub fn can_cancel(
        existing_appointments: &Vec<Appointment>,
        appointment_id: AppointmentId
    ) -> bool {
        let Some(appointment) = existing_appointments
            .iter()
            .find(|a| a.id() == appointment_id)
        else {
            return false;
        };

        if appointment.status() == &AppointmentStatus::Cancelled {
            return false;
        }
        
        if appointment.status() == &AppointmentStatus::Completed {
            return false;
        }
        
        let appointment_datetime = appointment.date().and_time(appointment.time());
        let now = Utc::now().naive_utc();
        
        if appointment_datetime < now {
            return false;
        }
        
        true
    }
}
