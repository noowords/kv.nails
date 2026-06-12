use async_trait::{ async_trait };

use crate::domain::{
    shared::{ TxContext },
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};

use super::super::shared::{ MySqlTxContext };

use super::{ MySqlProfileModel };

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
        ctx: &mut dyn TxContext,
        profile: &mut Profile
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

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
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Profile>, String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        let model: Option<MySqlProfileModel> = sqlx::query_as(
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

        match model {
            Some(model) => Ok(Some(Profile::try_from(model)?)),
            None => Ok(None),
        }
    }
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

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
        ctx: &mut dyn TxContext,
        profile: &mut Profile
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

        let model = MySqlProfileModel::from(&*profile);

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?
            WHERE user_id = ?
            "#
        )
            .bind(model.first_name())
            .bind(model.last_name())
            .bind(model.avatar_url())
            .bind(model.bio())
            .bind(model.user_id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| format!("Profile update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), String> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| "Invalid transaction context type".to_string())?;

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
