use sqlx::mysql::{ MySqlPool };

pub async fn create_mysql_pool() -> Result<MySqlPool, String> {
    MySqlPool::connect("mysql://root:root@localhost:3306/kvnails")
        .await
        .map_err(|e| format!("Database connection failed: {}", e))
}
