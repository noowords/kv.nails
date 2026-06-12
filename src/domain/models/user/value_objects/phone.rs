#[derive(Clone, Eq, PartialEq)]
pub struct UserPhone(String);

impl UserPhone {
    pub fn new(value: String) -> Result<Self, String> {
        Ok(Self(value))
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}
