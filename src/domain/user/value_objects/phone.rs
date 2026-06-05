#[derive(Clone, Eq, PartialEq)]
pub struct UserPhone(Option<String>);

impl UserPhone {
    pub fn new(value: Option<String>) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}
