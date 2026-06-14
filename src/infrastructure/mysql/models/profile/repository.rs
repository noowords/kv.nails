use async_trait::{ async_trait };

use crate::domain::shared::{ UnitOfWork };
use crate::domain::models::{
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};

use super::super::super::shared::{ MySqlTxContext };

use super::{ MySqlProfileRow };

pub struct MySqlProfileRepository;

impl MySqlProfileRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ProfileRepository for MySqlProfileRepository {
    async fn create(
        &self,
        uow: &mut dyn UnitOfWork,
        profile: &mut Profile
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(profile.user_id().value())
            .bind(profile.first_name())
            .bind(profile.last_name())
            .bind(profile.avatar_url())
            .bind(profile.bio())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Profile insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<Option<Profile>, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row: Option<MySqlProfileRow> = sqlx::query_as(
            r#"
            SELECT user_id, first_name, last_name, avatar_url, bio
            FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Find profile error: {}", e))?;

        match row {
            Some(row) => Ok(Some(Profile::try_from(row)?)),
            None => Ok(None),
        }
    }
    
    async fn exists(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<bool, String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM profiles
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Check profile existence error: {}", e))?;

        Ok(row.is_some())
    }
    
    async fn update(
        &self,
        uow: &mut dyn UnitOfWork,
        profile: &mut Profile
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        let row = MySqlProfileRow::from(&*profile);

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?
            WHERE user_id = ?
            "#
        )
            .bind(row.first_name())
            .bind(row.last_name())
            .bind(row.avatar_url())
            .bind(row.bio())
            .bind(row.user_id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Profile update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        uow: &mut dyn UnitOfWork,
        user_id: UserId
    ) -> Result<(), String> {
        let ctx = uow.ctx_mut()
            .as_any_mut()
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "UnitOfWork is not a MySqlUnitOfWork".to_string())?;

        sqlx::query(
            r#"
            DELETE FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Profile delete error: {}", e))?;

        Ok(())
    }
}
