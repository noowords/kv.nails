use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::{
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};

use super::super::models::{ ProfileRecord };

pub struct MySqlProfileRepository;

#[async_trait]
impl ProfileRepository for MySqlProfileRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        profile: &mut Profile
    ) -> Result<(), String> {
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
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Profile insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_user_id(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<Option<Profile>, String> {
        let profile_record: Option<ProfileRecord> = sqlx::query_as(
            r#"
            SELECT user_id, first_name, last_name, avatar_url, bio
            FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Find profile error: {}", e))?;

        match profile_record {
            Some(pr) => Ok(Some(Profile::try_from(pr)?)),
            None => Ok(None),
        }
    }
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<bool, String> {
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM profiles
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Check profile existence error: {}", e))?;

        Ok(row.is_some())
    }
    
    async fn update(
        tx: &mut Transaction<'_, MySql>,
        profile: &mut Profile
    ) -> Result<(), String> {
        let profile_record = ProfileRecord::from(&*profile);

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?
            WHERE user_id = ?
            "#
        )
            .bind(profile_record.first_name())
            .bind(profile_record.last_name())
            .bind(profile_record.avatar_url())
            .bind(profile_record.bio())
            .bind(profile_record.user_id())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Profile update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        user_id: UserId
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            DELETE FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Profile delete error: {}", e))?;

        Ok(())
    }
}
