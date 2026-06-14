use std::sync::{ Arc };

use crate::domain::shared::{ RepositoryProvider };
use crate::infrastructure::mysql::shared::{ MySqlUnitOfWorkFactory };
use crate::application::use_cases::{
    register_user::{ RegisterUserUseCase },
    create_master::{ CreateMasterUseCase },
    create_appointment::{ CreateAppointmentUseCase }
};
use crate::presentation::web::{ UseCases };

pub fn build_use_cases(uow_factory: Arc<MySqlUnitOfWorkFactory>, provider: Arc<dyn RepositoryProvider>) -> Arc<UseCases> {
    Arc::new(UseCases {
        register_user: Arc::new(
            RegisterUserUseCase::new(
                uow_factory.clone(),
                provider.user_repository(),
                provider.profile_repository()
            )
        ),
        create_master: Arc::new(
            CreateMasterUseCase::new(
                uow_factory.clone(),
                provider.master_repository()
            )
        ),
        create_appointment: Arc::new(
            CreateAppointmentUseCase::new(
                uow_factory.clone(),
                provider.appointment_repository()
            )
        )
    })
}
