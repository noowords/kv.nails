use crate::domain::{
    shared::{ UnitOfWork },
    user::{
        User, UserRepository,
        value_objects::{ UserPhone }
    },
    profile::{ Profile, ProfileRepository }
};

use super::{ RegisterUserCommand };

pub struct RegisterUserUseCase {
    user_repository: Box<dyn UserRepository>,
    profile_repository: Box<dyn ProfileRepository>,
    uow: Box<dyn UnitOfWork>
}

impl RegisterUserUseCase {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        profile_repository: Box<dyn ProfileRepository>,
        uow: Box<dyn UnitOfWork>
    ) -> Self {
        Self {
            user_repository,
            profile_repository,
            uow
        }
    }

    pub async fn execute(self, cmd: RegisterUserCommand) -> Result<(), String> {
        let mut user = User::new(
            None,
            None, 
            match cmd.user_phone {
                Some(p) => Some(UserPhone::new(p)?),
                None => None
            }
        );

        self.user_repository.create(&mut user).await?;

        let mut profile = Profile::new(
            user.id(),
            cmd.profile_first_name,
            cmd.profile_last_name,
            cmd.profile_avatar_url,
            cmd.profile_bio
        );

        self.profile_repository.create(&mut profile).await?;

        self.uow.commit().await?;

        Ok(())
    }
}
