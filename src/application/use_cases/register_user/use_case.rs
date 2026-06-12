use std::sync::{ Arc };

use crate::domain::shared::{ TxContext };
use crate::domain::models::{
    user::{
        User, UserRepository,
        value_objects::{ UserPhone }
    },
    profile::{ Profile, ProfileRepository }
};

use super::{ RegisterUserCommand };

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    profile_repository: Arc<dyn ProfileRepository>
}

impl RegisterUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        profile_repository: Arc<dyn ProfileRepository>
    ) -> Self {
        Self {
            user_repository,
            profile_repository
        }
    }

    pub async fn execute(
        &self,
        ctx: &mut dyn TxContext,
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

        self.user_repository.create(ctx, &mut user).await?;

        let mut profile = Profile::new(
            user.id(),
            cmd.profile_first_name,
            cmd.profile_last_name,
            cmd.profile_avatar_url,
            cmd.profile_bio
        );

        self.profile_repository.create(ctx, &mut profile).await?;

        Ok(())
    }
}
