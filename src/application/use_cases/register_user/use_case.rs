use std::sync::{ Arc };

use crate::domain::models::{
    user::{
        User, UserRepository,
        value_objects::{ UserPhone }
    },
    profile::{ Profile, ProfileRepository }
};

use super::super::super::shared::{ UnitOfWorkFactory };

use super::{ RegisterUserCommand };

pub struct RegisterUserUseCase {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    user_repository: Arc<dyn UserRepository>,
    profile_repository: Arc<dyn ProfileRepository>
}

impl RegisterUserUseCase {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        user_repository: Arc<dyn UserRepository>,
        profile_repository: Arc<dyn ProfileRepository>
    ) -> Self {
        Self {
            uow_factory,
            user_repository,
            profile_repository
        }
    }

    pub async fn execute(
        &self,
        cmd: RegisterUserCommand
    ) -> Result<(), String> {
        let mut uow = self.uow_factory.begin().await?;

        let mut user = User::new(
            None,
            None, 
            match cmd.user_phone {
                Some(p) => Some(UserPhone::new(p)?),
                None => None
            }
        );

        self.user_repository.create(&mut *uow, &mut user).await?;

        let mut profile = Profile::new(
            user.id(),
            cmd.profile_first_name,
            cmd.profile_last_name,
            cmd.profile_avatar_url,
            cmd.profile_bio
        );

        self.profile_repository.create(&mut *uow, &mut profile).await?;

        uow.commit().await?;

        Ok(())
    }
}
