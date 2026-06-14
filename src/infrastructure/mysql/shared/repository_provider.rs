use std::sync::{ Arc };

use crate::domain::{
    shared::{ RepositoryProvider },
    models::{
        user::{ UserRepository },
        profile::{ ProfileRepository },
        master::{ MasterRepository },
        appointment::{ AppointmentRepository }
    }
};

use super::super::models::{
    user::{ MySqlUserRepository },
    profile::{ MySqlProfileRepository },
    master::{ MySqlMasterRepository },
    appointment::{ MySqlAppointmentRepository }
};

pub struct MySqlRepositoryProvider {
    user_repository: Arc<MySqlUserRepository>,
    profile_repository: Arc<MySqlProfileRepository>,
    master_repository: Arc<MySqlMasterRepository>,
    appointment_repository: Arc<MySqlAppointmentRepository>
}

impl MySqlRepositoryProvider {
    pub fn new() -> Self {
        Self {
            user_repository: Arc::new(MySqlUserRepository::new()),
            profile_repository: Arc::new(MySqlProfileRepository::new()),
            master_repository: Arc::new(MySqlMasterRepository::new()),
            appointment_repository: Arc::new(MySqlAppointmentRepository::new())
        }
    }
}

impl RepositoryProvider for MySqlRepositoryProvider {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
    
    fn profile_repository(&self) -> Arc<dyn ProfileRepository> {
        self.profile_repository.clone()
    }
    
    fn master_repository(&self) -> Arc<dyn MasterRepository> {
        self.master_repository.clone()
    }
    
    fn appointment_repository(&self) -> Arc<dyn AppointmentRepository> {
        self.appointment_repository.clone()
    }
}
