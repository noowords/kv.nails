use async_trait::{ async_trait };
use sqlx::{ MySql, Transaction };

use crate::domain::user::{
    User, UserRepository,
    value_objects::{ UserId }
};

use super::super::models::{ UserRecord };

pub struct MySqlUserRepository;

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(
        tx: &mut Transaction<'_, MySql>,
        user: &mut User
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO users (phone, role, created_at)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(user.phone().value())
            .bind(user.role().as_str())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("User insert error: {}", e))?;

        Ok(())
    }
    
    async fn get_by_id(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<Option<User>, String> {
        let user_record: Option<UserRecord> = sqlx::query_as(
            r#"
            SELECT id, role, phone
            FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| format!("Find user error: {}", e))?;

        match user_record {
            Some(ur) => Ok(Some(User::try_from(ur)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<bool, String> {
        unimplemented!()
    }

    async fn update(
        tx: &mut Transaction<'_, MySql>,
        user: &mut User
    ) -> Result<(), String> {
        let user_record = UserRecord::from(&*user);

        sqlx::query("UPDATE users SET phone = ?, role = ? WHERE id = ?")
            .bind(user_record.phone())
            .bind(user_record.role())
            .bind(user_record.id())
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("User update error: {}", e))?;

        Ok(())
    }
    
    async fn remove(
        tx: &mut Transaction<'_, MySql>,
        id: UserId
    ) -> Result<(), String> {
        unimplemented!()
    }
}
