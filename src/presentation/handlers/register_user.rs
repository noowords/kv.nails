use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::infrastructure::mysql::{ MySqlUnitOfWork };
use crate::application::use_cases::register_user::{ RegisterUserCommand, RegisterUserUseCase };

use super::super::{ AppState };

pub async fn register_user(
    State(state): State<AppState>
) -> Result<(), (StatusCode, String)> {
    let mut uow = MySqlUnitOfWork::begin(state.db_pool())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let cmd = RegisterUserCommand::new(
        Some(String::from("00000000000")),
        String::from("Иван"),
        String::from("Иванов"),
        None,
        None
    );

    if let Err(e) = RegisterUserUseCase::execute(&mut uow, cmd).await {
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
