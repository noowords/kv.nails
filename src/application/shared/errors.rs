use thiserror::{ Error };

use crate::domain::shared::{ SharedDomainError };

#[derive(Debug, Error)]
pub enum SharedApplicationError {
    #[error(transparent)]
    SharedDomainError(#[from] SharedDomainError)
}
