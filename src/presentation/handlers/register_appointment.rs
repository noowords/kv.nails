use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::domain::user::value_objects::{ UserId };
use crate::infrastructure::mysql::{ MySqlUnitOfWork };
use crate::application::use_cases::create_appointment::{ CreateAppointmentCommand, CreateAppointmentUseCase };

use super::super::{ AppState };

pub async fn register_appointment(
    State(state): State<AppState>
) -> Result<(), (StatusCode, String)> {
    let mut uow = MySqlUnitOfWork::begin(state.db_pool())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let cmd = CreateAppointmentCommand::new(
        UserId::from(Uuid::now_v7()),
        UserId::from(Uuid::now_v7()),
        NaiveDate::from_ymd_opt(2026, 06, 14).unwrap(),
        NaiveTime::from_hms_opt(09, 00, 00).unwrap()
    );

    if let Err(e) = CreateAppointmentUseCase::execute(&mut uow, cmd).await {
        uow.rollback()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        return Err((StatusCode::BAD_REQUEST, e));
    }

    uow.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(())
}
