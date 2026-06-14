use thiserror::{ Error };

use crate::domain::{
    shared::{ SharedDomainError },
    models::appointment::{ AppointmentModelDomainError }
};

use super::super::super::shared::{ SharedApplicationError };

#[derive(Debug, Error)]
pub enum CreateAppointmentUseCaseApplicationError {
    #[error(transparent)]
    SharedApplicationError(#[from] SharedApplicationError),

    #[error(transparent)]
    AppointmentModelDomainError(#[from] AppointmentModelDomainError),

    #[error(transparent)]
    SharedDomainError(#[from] SharedDomainError)
}
