use std::sync::{ Arc };
use sqlx::{ MySqlPool };

use crate::application::use_cases::{
    create_appointment::{ CreateAppointmentUseCase },
    create_master::{ CreateMasterUseCase },
    register_user::{ RegisterUserUseCase }
};

#[derive(Clone)]
pub struct AppState {
    db_pool: MySqlPool,
    
    pub create_appointment_uc: Arc<CreateAppointmentUseCase>,
    pub create_master_uc: Arc<CreateMasterUseCase>,
    pub register_user_uc: Arc<RegisterUserUseCase>
}

impl AppState {
    pub fn new(
        db_pool: MySqlPool,
        create_appointment_uc: Arc<CreateAppointmentUseCase>,
        create_master_uc: Arc<CreateMasterUseCase>,
        register_user_uc: Arc<RegisterUserUseCase>
    ) -> Self {
        Self {
            db_pool,
            create_appointment_uc,
            create_master_uc,
            register_user_uc
        }
    }

    pub fn db_pool(&self) -> &MySqlPool {
        &self.db_pool
    }
}
