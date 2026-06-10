use sqlx::{ MySqlPool };

#[derive(Clone)]
pub struct AppState {
    db_pool: MySqlPool
}

impl AppState {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub fn db_pool(&self) -> &MySqlPool {
        &self.db_pool
    }
}
