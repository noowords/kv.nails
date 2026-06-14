use axum::{
    extract::{ State },
    http::{ StatusCode },
    Json
};

use crate::application::use_cases::register_user::{ RegisterUserCommand };

use super::super::{ AppState, AppError };

pub async fn register_user(
    State(state): State<AppState>,
    Json(cmd): Json<RegisterUserCommand>
) -> Result<StatusCode, AppError> {
    state.ucs.register_user.execute(cmd).await?;

    Ok(StatusCode::CREATED)
}
