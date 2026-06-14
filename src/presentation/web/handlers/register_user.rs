use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::application::use_cases::register_user::{ RegisterUserCommand };

use super::super::{ AppState };

pub async fn register_user(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let cmd = RegisterUserCommand::new(
        Some(String::from("00000000000")),
        String::from("Иван"),
        String::from("Иванов"),
        None,
        None
    );

    state.ucs.register_user.execute(cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::CREATED)
}
