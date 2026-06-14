use super::super::errors::{ AppointmentModelDomainError };

#[derive(Clone, PartialEq)]
pub enum AppointmentStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed
}

impl AppointmentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppointmentStatus::Pending => "pending",
            AppointmentStatus::Confirmed => "confirmed",
            AppointmentStatus::Cancelled => "cancelled",
            AppointmentStatus::Completed => "completed"
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(AppointmentStatus::Pending),
            "confirmed" => Some(AppointmentStatus::Confirmed),
            "cancelled" => Some(AppointmentStatus::Cancelled),
            "completed" => Some(AppointmentStatus::Completed),
            _ => None
        }
    }
}

impl From<AppointmentStatus> for String {
    fn from(role: AppointmentStatus) -> Self {
        role.as_str().to_string()
    }
}

impl TryFrom<&str> for AppointmentStatus {
    type Error = AppointmentModelDomainError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        AppointmentStatus::from_str(s).ok_or_else(|| AppointmentModelDomainError::InvalidStatus(s.to_string()))
    }
}
