use thiserror::{ Error };

use crate::domain::{
    shared::{ SharedDomainError },
    models::{
        user::{ UserModelDomainError },
        profile::{ ProfileModelDomainError }
    }
};

use super::super::super::shared::{ SharedApplicationError };

#[derive(Debug, Error)]
pub enum RegisterUserUseCaseApplicationError {
    #[error(transparent)]
    SharedApplicationError(#[from] SharedApplicationError),

    #[error(transparent)]
    UserModelDomainError(#[from] UserModelDomainError),

    #[error(transparent)]
    ProfileModelDomainError(#[from] ProfileModelDomainError),

    #[error(transparent)]
    SharedDomainError(#[from] SharedDomainError)
}
