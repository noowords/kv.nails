use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum ProfileModelDomainError {
    #[error("Ошибка базы данных при обработке записи: {0}.")]
    DatabaseError(String)
}
