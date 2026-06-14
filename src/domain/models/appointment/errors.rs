use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum AppointmentModelDomainError {
    #[error("Неверный формат идентификатора записи. Ожидается валидный UUID.")]
    InvalidId,

    #[error("Передан неизвестный статус записи: {0}.")]
    InvalidStatus(String),

    #[error("Ошибка базы данных при обработке записи: {0}.")]
    DatabaseError(String)
}
