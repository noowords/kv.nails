use super::super::errors::{ UserModelDomainError };

#[derive(Clone, Eq, PartialEq)]
pub enum UserRole {
    Admin,
    Master,
    Client
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Master => "master",
            UserRole::Client => "client"
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(UserRole::Admin),
            "master" => Some(UserRole::Master),
            "client" => Some(UserRole::Client),
            _ => None
        }
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        role.as_str().to_string()
    }
}

impl TryFrom<&str> for UserRole {
    type Error = UserModelDomainError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        UserRole::from_str(s).ok_or_else(|| UserModelDomainError::InvalidRole(s.to_string()))
    }
}
