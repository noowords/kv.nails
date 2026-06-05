use crate::domain::{
    user::{ User, UserRepository },
    profile::{ Profile, ProfileRepository }
};
use crate::infrastructure::mysql::{
    MySqlUnitOfWork,
    repositories::{ MySqlUserRepository, MySqlProfileRepository }
};

use super::{ RegisterUserCommand };

pub struct RegisterUserUseCase;

impl RegisterUserUseCase {
    pub async fn execute(
        uow: &mut MySqlUnitOfWork<'_>,
        cmd: RegisterUserCommand
    ) -> Result<(), String> {
        let mut user = User::new(cmd.phone);
        MySqlUserRepository::create(&mut uow.tx(), &mut user).await?;

        let mut profile = Profile::new(user.id());
        MySqlProfileRepository::create(&mut uow.tx(), &mut profile).await?;

        Ok(())
    }
}
