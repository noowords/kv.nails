use super::super::user::value_objects::{ UserId };

#[derive(Clone, Eq, PartialEq)]
pub struct Profile {
    user_id: UserId,
    first_name: String,
    last_name: String,
    avatar_url: Option<String>,
    bio: Option<String>
}

impl Profile {
    pub fn new(
        user_id: UserId,
        first_name: String,
        last_name: String,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self {
            user_id,
            first_name,
            last_name,
            avatar_url,
            bio
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
}
