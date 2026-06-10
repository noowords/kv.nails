mod domain;
mod infrastructure;
mod application;
mod presentation;

use crate::infrastructure::mysql::{ create_mysql_pool };
use crate::presentation::{ AppState, create_router, run };

#[tokio::main]
async fn main() {
    let db_pool = create_mysql_pool("mysql://root:root@localhost:3306/kvnails").await;
    let state = AppState::new(db_pool);
    let app = create_router(state);
    
    run(app).await;
}
