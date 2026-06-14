use std::sync::{ Arc };

use crate::application::use_cases::{
    register_user::{ RegisterUserUseCase },
    create_master::{ CreateMasterUseCase },
    create_appointment::{ CreateAppointmentUseCase }
};

pub struct UseCases {
    pub register_user: Arc<RegisterUserUseCase>,
    pub create_master: Arc<CreateMasterUseCase>,
    pub create_appointment: Arc<CreateAppointmentUseCase>
}

#[derive(Clone)]
pub struct AppState {
    pub ucs: Arc<UseCases>
}

impl AppState {
    pub fn new(ucs: Arc<UseCases>) -> Self {
        Self { ucs }
    }
}
