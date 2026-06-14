use super::super::errors::{ UserModelDomainError };

#[derive(Clone, Eq, PartialEq)]
pub struct UserPhone(String);

impl UserPhone {
    pub fn new(value: String) -> Result<Self, UserModelDomainError> {
        if false { return Err(UserModelDomainError::InvalidPhoneFormat) };

        Ok(Self(value))
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}
