use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum UserModelDomainError {
    #[error("Неверный формат идентификатора записи. Ожидается валидный UUID.")]
    InvalidId,

    #[error("Передана неизвестная роль пользователя: {0}.")]
    InvalidRole(String),
    
    #[error("Неверный формат номера телефона")]
    InvalidPhoneFormat,

    #[error("Ошибка базы данных при обработке записи: {0}.")]
    DatabaseError(String)
}
