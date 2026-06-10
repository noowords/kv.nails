use sqlx::{
    MySqlPool,
    mysql::{ MySqlPoolOptions }
};

pub async fn create_mysql_pool(url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .unwrap()
}
