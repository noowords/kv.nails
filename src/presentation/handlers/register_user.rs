use axum::{
    extract::{ State },
    http::{ StatusCode }
};

use crate::infrastructure::mysql::shared::{ MySqlTxContext };
use crate::application::use_cases::register_user::{ RegisterUserCommand };

use super::super::{ AppState };

pub async fn register_user(
    State(state): State<AppState>
) -> Result<StatusCode, (StatusCode, String)> {
    let tx = state.db_pool().begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut ctx = MySqlTxContext::new(tx);

    let cmd = RegisterUserCommand::new(
        Some(String::from("00000000000")),
        String::from("Иван"),
        String::from("Иванов"),
        None,
        None
    );

    state.register_user_uc.execute(&mut ctx, cmd)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    ctx.tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
