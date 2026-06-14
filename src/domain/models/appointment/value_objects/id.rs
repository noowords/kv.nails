use uuid::{ Uuid };

use super::super::errors::{ AppointmentModelDomainError };

#[derive(Copy, Clone, PartialEq)]
pub struct AppointmentId(Uuid);

impl AppointmentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
    
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Uuid> for AppointmentId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<AppointmentId> for Uuid {
    fn from(id: AppointmentId) -> Self {
        id.0
    }
}

impl std::str::FromStr for AppointmentId {
    type Err = AppointmentModelDomainError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(AppointmentId)
            .map_err(|_| AppointmentModelDomainError::InvalidId)
    }
}

impl std::fmt::Display for AppointmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AppointmentId {
    fn default() -> Self {
        Self::new()
    }
}
