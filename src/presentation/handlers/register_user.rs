use std::sync::{ Arc };
use tokio::sync::Mutex;
use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::infrastructure::mysql::{
    shared::{ MySqlUnitOfWork },
    user::{ MySqlUserRepository },
    profile::{ MySqlProfileRepository }
};
use crate::application::use_cases::register_user::{ RegisterUserCommand, RegisterUserUseCase };

use super::super::{ AppState };

pub async fn register_user(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let tx = state.db_pool().begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let shared_tx = Arc::new(Mutex::new(Some(tx)));

    let user_repository = MySqlUserRepository::new(shared_tx.clone()); 
    let profile_repository = MySqlProfileRepository::new(shared_tx.clone()); 
    let uow = MySqlUnitOfWork::new(shared_tx);

    let uc = RegisterUserUseCase::new(
        Box::new(user_repository),
        Box::new(profile_repository),
        Box::new(uow)
    );

    let cmd = RegisterUserCommand::new(
        Some(String::from("00000000000")),
        String::from("Иван"),
        String::from("Иванов"),
        None,
        None
    );

    uc.execute(cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::CREATED)
}
