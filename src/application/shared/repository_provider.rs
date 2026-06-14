use std::sync::{ Arc };

use crate::domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};

pub trait RepositoryProvider: Send + Sync {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    
    fn profile_repository(&self) -> Arc<dyn ProfileRepository>;
    
    fn master_repository(&self) -> Arc<dyn MasterRepository>;
    
    fn appointment_repository(&self) -> Arc<dyn AppointmentRepository>;
}
