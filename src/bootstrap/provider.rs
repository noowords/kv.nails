use std::sync::{ Arc };

use crate::infrastructure::mysql::shared::{ MySqlRepositoryProvider };

pub async fn create_mysql_provider() -> Arc<MySqlRepositoryProvider> {
    Arc::new(MySqlRepositoryProvider::new())
}
