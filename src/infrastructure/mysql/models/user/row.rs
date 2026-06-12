use uuid::{ Uuid };
use sqlx::{ FromRow };

use crate::domain::models::user::{
    User,
    value_objects::{ UserId, UserRole, UserPhone }
};

#[derive(FromRow)]
pub struct MySqlUserRow {
    id: Uuid,
    role: String,
    phone: Option<String>
}

impl MySqlUserRow {
    pub fn new(
        id: Uuid,
        role: String,
        phone: Option<String>
    ) -> Self {
        Self {
            id,
            role,
            phone
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    
    pub fn role(&self) -> &str {
        &self.role
    }
    
    pub fn phone(&self) -> &Option<String> {
        &self.phone
    }
}

impl TryFrom<MySqlUserRow> for User {
    type Error = String;
    
    fn try_from(record: MySqlUserRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            Some(UserId::from(record.id)),
            Some(UserRole::from_str(&record.role)
                .ok_or_else(|| format!("Unknown role: {}", record.role))?),
             match record.phone {
                Some(p) => Some(UserPhone::new(p)?),
                None => None
            }
        ))
    }
}

impl From<&User> for MySqlUserRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().value(),
            role: user.role().as_str().to_string(),
            phone: user.phone().as_ref().map(|p| p.value().clone())
        }
    }
}
