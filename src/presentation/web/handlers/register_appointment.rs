use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::domain::models::user::value_objects::{ UserId };
use crate::application::use_cases::create_appointment::{ CreateAppointmentCommand };

use super::super::{ AppState };

pub async fn register_appointment(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let cmd = CreateAppointmentCommand::new(
        UserId::from(Uuid::now_v7()),
        UserId::from(Uuid::now_v7()),
        NaiveDate::from_ymd_opt(2026, 06, 14).unwrap(),
        NaiveTime::from_hms_opt(09, 00, 00).unwrap()
    );

    state.ucs.create_appointment.execute(cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    Ok(StatusCode::CREATED)
}
