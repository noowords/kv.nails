use axum::{
    http::StatusCode,
    response::{ IntoResponse, Response }
};

use crate::domain::models::{
    user::{ UserModelDomainError },
    master::{ MasterModelDomainError },
    appointment::{ AppointmentModelDomainError }
};
use crate::application::use_cases::{
    register_user::{ RegisterUserUseCaseApplicationError },
    create_master::{ CreateMasterUseCaseApplicationError },
    create_appointment::{ CreateAppointmentUseCaseApplicationError }
};

pub enum AppError {
    RegisterUser(RegisterUserUseCaseApplicationError),
    CreateMaster(CreateMasterUseCaseApplicationError),
    CreateAppointment(CreateAppointmentUseCaseApplicationError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::RegisterUser(err) => match err {
                RegisterUserUseCaseApplicationError::UserModelDomainError(domain_err) => match domain_err {
                    UserModelDomainError::DatabaseError(tech_details) => {
                        eprintln!("[DATABASE ERROR - USER]: {}", tech_details);
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                    _ => StatusCode::BAD_REQUEST
                }
                RegisterUserUseCaseApplicationError::ProfileModelDomainError(_) => StatusCode::BAD_REQUEST,
                RegisterUserUseCaseApplicationError::SharedDomainError(err) => {
                    eprintln!("[TRANSACTION ERROR - USER DOMAIN]: {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                RegisterUserUseCaseApplicationError::SharedApplicationError(err) => {
                    eprintln!("[TRANSACTION ERROR - USER APP]: {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            AppError::CreateMaster(err) => match err {
                CreateMasterUseCaseApplicationError::MasterModelDomainError(domain_err) => match domain_err {
                    MasterModelDomainError::DatabaseError(tech_details) => {
                        eprintln!("[DATABASE ERROR - MASTER]: {}", tech_details);
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                    _ => StatusCode::BAD_REQUEST
                }
                CreateMasterUseCaseApplicationError::SharedApplicationError(app_err) => {
                    eprintln!("[TRANSACTION ERROR - MASTER APP]: {:?}", app_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                CreateMasterUseCaseApplicationError::SharedDomainError(dom_err) => {
                    eprintln!("[TRANSACTION ERROR - MASTER DOMAIN]: {:?}", dom_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            AppError::CreateAppointment(err) => match err {
                CreateAppointmentUseCaseApplicationError::AppointmentModelDomainError(domain_err) => match domain_err {
                    AppointmentModelDomainError::DatabaseError(tech_details) => {
                        eprintln!("[DATABASE ERROR - APPOINTMENT]: {}", tech_details);
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                    _ => StatusCode::BAD_REQUEST
                }
                CreateAppointmentUseCaseApplicationError::SharedApplicationError(app_err) => {
                    eprintln!("[TRANSACTION ERROR - APPOINTMENT APP]: {:?}", app_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                CreateAppointmentUseCaseApplicationError::SharedDomainError(dom_err) => {
                    eprintln!("[TRANSACTION ERROR - APPOINTMENT DOMAIN]: {:?}", dom_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        };

        status.into_response()
    }
}

impl From<RegisterUserUseCaseApplicationError> for AppError {
    fn from(err: RegisterUserUseCaseApplicationError) -> Self {
        AppError::RegisterUser(err)
    }
}

impl From<CreateMasterUseCaseApplicationError> for AppError {
    fn from(err: CreateMasterUseCaseApplicationError) -> Self {
        AppError::CreateMaster(err)
    }
}

impl From<CreateAppointmentUseCaseApplicationError> for AppError {
    fn from(err: CreateAppointmentUseCaseApplicationError) -> Self {
        AppError::CreateAppointment(err)
    }
}
