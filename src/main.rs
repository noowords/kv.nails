mod domain;
mod infrastructure;
mod application;
mod presentation;

use sqlx::mysql::{ MySqlPoolOptions };

use crate::presentation::{ AppState, create_router, run };

#[tokio::main]
async fn main() {
    let db_pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:root@localhost:3306/kvnails")
        .await
        .unwrap();
    let state = AppState::new(db_pool);
    let app = create_router(state);
    
    run(app).await;
}
