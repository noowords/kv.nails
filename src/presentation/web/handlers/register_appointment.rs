use axum::{
    extract::{ State },
    http::{ StatusCode },
    Json
};

use crate::application::use_cases::create_appointment::{ CreateAppointmentCommand };

use super::super::{ AppState, AppError };

pub async fn register_appointment(
    State(state): State<AppState>,
    Json(cmd): Json<CreateAppointmentCommand>
) -> Result<StatusCode, AppError> {
    state.ucs.create_appointment.execute(cmd).await?;
    
    Ok(StatusCode::CREATED)
}
