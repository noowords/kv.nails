use axum::{
    Router,
    routing::{ get, post }
};

use super::{ AppState, handlers };

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/users", post(handlers::register_user))
        .route("/appointments", post(handlers::register_appointment))
        .with_state(state)
}
