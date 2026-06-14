use std::sync::{ Arc };
use sqlx::mysql::{ MySqlPool };

use crate::infrastructure::mysql::shared::{ MySqlUnitOfWorkFactory };

pub fn create_mysql_unit_of_work_factory(db_pool: MySqlPool) -> Arc<MySqlUnitOfWorkFactory> {
    Arc::new(MySqlUnitOfWorkFactory::new(db_pool))
}
