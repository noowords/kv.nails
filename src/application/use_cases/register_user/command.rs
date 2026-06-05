use crate::domain::user::value_objects::{ UserPhone };

pub struct RegisterUserCommand {
    pub phone: UserPhone
}

impl RegisterUserCommand {
    pub fn new(
        phone: UserPhone
    ) -> Self {
        Self { phone }
    }
}
