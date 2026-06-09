use crate::domain::{
    user::{
        User, UserRepository,
        value_objects::{ UserPhone }
    },
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
        let mut user = User::new(
            None,
            None, 
            match cmd.user_phone {
                Some(p) => Some(UserPhone::new(p)?),
                None => None
            }
        );
        MySqlUserRepository::create(&mut uow.tx(), &mut user).await?;

        let mut profile = Profile::new(
            user.id(),
            cmd.profile_first_name,
            cmd.profile_last_name,
            cmd.profile_avatar_url,
            cmd.profile_bio
        );
        MySqlProfileRepository::create(&mut uow.tx(), &mut profile).await?;

        Ok(())
    }
}
