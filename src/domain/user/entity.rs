use super::value_objects::{ UserId, UserRole, UserPhone };

#[derive(Clone, Eq, PartialEq)]
pub struct User {
    id: UserId,
    role: UserRole,
    phone: UserPhone
}

impl User {
    pub fn new(phone: UserPhone) -> Self {
        Self {
            id: UserId::new(),
            role: UserRole::Client,
            phone
        }
    }

    pub fn from_record(id: UserId, role: UserRole, phone: UserPhone) -> Self {
        Self {
            id: id,
            role,
            phone
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn phone(&self) -> &UserPhone {
        &self.phone
    }

    pub(crate) fn set_role(&mut self, role: UserRole) {
        self.role = role;
    }

    pub(crate) fn set_phone(&mut self, phone: UserPhone) {
        self.phone = phone;
    }
}
