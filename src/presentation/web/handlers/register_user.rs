use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::application::use_cases::register_user::{ RegisterUserCommand };

use super::super::{ AppState, AppError };

pub async fn register_user(
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    let cmd = RegisterUserCommand::new(
        Some(String::from("00000000000")),
        String::from("Иван"),
        String::from("Иванов"),
        None,
        None
    );

    state.ucs.register_user.execute(cmd).await?;

    Ok(StatusCode::CREATED)
}
