use std::sync::{ Arc };
use tokio::sync::{ Mutex };
use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::{
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};

use super::{ MySqlProfileModel };

pub struct MySqlProfileRepository {
    tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>
}

impl MySqlProfileRepository {
    pub fn new(tx: Arc<Mutex<Option<Transaction<'static, MySql>>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl ProfileRepository for MySqlProfileRepository {
    async fn create(&self, profile: &mut Profile) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

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
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Profile>, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let profile_record: Option<MySqlProfileModel> = sqlx::query_as(
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
    
    async fn exists(&self, user_id: UserId) -> Result<bool, String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

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
    
    async fn update(&self, profile: &mut Profile) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

        let profile_record = MySqlProfileModel::from(&*profile);

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
    
    async fn remove(&self, user_id: UserId) -> Result<(), String> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.as_mut()
            .ok_or_else(|| "The transaction has already been completed".to_string())?;

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
