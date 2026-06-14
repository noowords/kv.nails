use thiserror::{ Error };

#[derive(Debug, Error)]
pub enum MasterModelDomainError {
    #[error("Данные расписания мастера повреждены или несовместимы: {0}.")]
    CorruptedSchedule(String),

    #[error("Ошибка базы данных при обработке записи: {0}.")]
    DatabaseError(String)
}
