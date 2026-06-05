use super::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};

pub trait UnitOfWork {
    type UserRepository: UserRepository;
    type ProfileRepository: ProfileRepository;
    type MasterRepository: MasterRepository;
    type AppointmentRepository: AppointmentRepository;

    fn user_repository(&mut self) -> &mut Self::UserRepository;
    fn profile_repository(&mut self) -> &mut Self::ProfileRepository;
    fn master_repository(&mut self) -> &mut Self::MasterRepository;
    fn appointment_repository(&mut self) -> &mut Self::AppointmentRepository;

    async fn begin(&mut self) -> Result<(), String>;
    async fn commit(&mut self) -> Result<(), String>;
    async fn rollback(&mut self) -> Result<(), String>;
}
