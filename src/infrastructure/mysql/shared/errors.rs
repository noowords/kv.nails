use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum SharedMySqlInfrastructureError {
    #[error("Критический сбой подключения к MySQL: {0}")]
    ConnectionError(String),

    #[error("Сбой при выполнении SQL-запроса или транзакции: {0}")]
    QueryError(String)
}

impl SharedMySqlInfrastructureError {
    pub fn from_sqlx(err: sqlx::Error) -> Self {
        let err_msg = err.to_string();
        if err_msg.contains("pool") || err_msg.contains("connection") {
            Self::ConnectionError(err_msg)
        } else {
            Self::QueryError(err_msg)
        }
    }
}
