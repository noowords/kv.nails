use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum SharedDomainError {
    #[error("Не удалось открыть транзакцию базы данных: {0}.")]
    TransactionBeginFailed(String),

    #[error("Не удалось зафиксировать (commit) изменения в базе данных: {0}.")]
    TransactionCommitFailed(String),

    #[error("Не удалось откатить (rollback) транзакцию: {0}.")]
    TransactionRollbackFailed(String)
}
