use thiserror::{ Error };

use crate::domain::{
    shared::{ SharedDomainError },
    models::master::{ MasterModelDomainError }
};

use super::super::super::shared::{ SharedApplicationError };

#[derive(Debug, Error)]
pub enum CreateMasterUseCaseApplicationError {
    #[error(transparent)]
    SharedApplicationError(#[from] SharedApplicationError),

    #[error(transparent)]
    MasterModelDomainError(#[from] MasterModelDomainError),

    #[error(transparent)]
    SharedDomainError(#[from] SharedDomainError)
}
