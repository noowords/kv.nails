use std::sync::{ Arc };
use tokio::sync::Mutex;
use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::domain::user::value_objects::{ UserId };
use crate::infrastructure::mysql::{
    shared::{ MySqlUnitOfWork },
    appointment::{ MySqlAppointmentRepository }
};
use crate::application::use_cases::create_appointment::{ CreateAppointmentCommand, CreateAppointmentUseCase };

use super::super::{ AppState };

pub async fn register_appointment(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let tx = state.db_pool().begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let shared_tx = Arc::new(Mutex::new(Some(tx)));

    let appointment_repository = MySqlAppointmentRepository::new(shared_tx.clone()); 
    let uow = MySqlUnitOfWork::new(shared_tx);

    let uc = CreateAppointmentUseCase::new(
        Box::new(appointment_repository),
        Box::new(uow)
    );

    let cmd = CreateAppointmentCommand::new(
        UserId::from(Uuid::now_v7()),
        UserId::from(Uuid::now_v7()),
        NaiveDate::from_ymd_opt(2026, 06, 14).unwrap(),
        NaiveTime::from_hms_opt(09, 00, 00).unwrap()
    );

    uc.execute(cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::CREATED)
}
