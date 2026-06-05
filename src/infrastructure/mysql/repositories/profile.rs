use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::{
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};

pub struct MySqlProfileRepository;

#[async_trait]
impl ProfileRepository for MySqlProfileRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        profile: &mut Profile
    ) -> Result<(), String> {
        Ok(())
        // unimplemented!()
    }
    
    async fn get_by_user_id(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<Option<Profile>, String> {
        unimplemented!()
    }
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<bool, String> {
        unimplemented!()
    }
    
    async fn update(
        tx: &mut Transaction<'_, MySql>,
        profile: &mut Profile
    ) -> Result<(), String> {
        unimplemented!()
    }
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<(), String> {
        unimplemented!()
    }
}
