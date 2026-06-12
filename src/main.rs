mod domain;
mod infrastructure;
mod application;
mod presentation;

use std::sync::{ Arc };
use sqlx::{ MySqlPool };

use crate::infrastructure::mysql::{
    user::{ MySqlUserRepository },
    profile::{ MySqlProfileRepository },
    master::{ MySqlMasterRepository },
    appointment::{ MySqlAppointmentRepository }
};
use crate::application::use_cases::{
    create_appointment::{ CreateAppointmentUseCase },
    create_master::{ CreateMasterUseCase },
    register_user::{ RegisterUserUseCase }
};
use crate::presentation::{ AppState, create_router, run };

#[tokio::main]
async fn main() {
    let db_pool = MySqlPool::connect("mysql://root:root@localhost:3306/kvnails").await.unwrap();
    
    let user_repository = Arc::new(MySqlUserRepository::new());
    let profile_repository = Arc::new(MySqlProfileRepository::new());
    let master_repository = Arc::new(MySqlMasterRepository::new());
    let appointment_repository = Arc::new(MySqlAppointmentRepository::new());
    
    let create_appointment_uc = Arc::new(CreateAppointmentUseCase::new(appointment_repository));
    let create_master_uc = Arc::new(CreateMasterUseCase::new(master_repository));
    let register_user_uc = Arc::new(RegisterUserUseCase::new(user_repository, profile_repository));

    let state = AppState::new(
        db_pool,
        create_appointment_uc,
        create_master_uc,
        register_user_uc
    );

    let app = create_router(state);
    
    run(app).await;
}
