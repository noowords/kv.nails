use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::domain::models::user::value_objects::{ UserId };
use crate::infrastructure::mysql::shared::{ MySqlTxContext };
use crate::application::use_cases::create_appointment::{ CreateAppointmentCommand };

use super::super::{ AppState };

pub async fn register_appointment(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let tx = state.db_pool().begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut ctx = MySqlTxContext::new(tx);

    let cmd = CreateAppointmentCommand::new(
        UserId::from(Uuid::now_v7()),
        UserId::from(Uuid::now_v7()),
        NaiveDate::from_ymd_opt(2026, 06, 14).unwrap(),
        NaiveTime::from_hms_opt(09, 00, 00).unwrap()
    );

    state.create_appointment_uc.execute(&mut ctx, cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    ctx.tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::CREATED)
}
