pub struct RegisterUserCommand {
    pub user_phone: Option<String>,
    pub profile_first_name: String,
    pub profile_last_name: String,
    pub profile_avatar_url: Option<String>,
    pub profile_bio: Option<String>
}

impl RegisterUserCommand {
    pub fn new(
        user_phone: Option<String>,
        profile_first_name: String,
        profile_last_name: String,
        profile_avatar_url: Option<String>,
        profile_bio: Option<String>
    ) -> Self {
        Self {
            user_phone,
            profile_first_name,
            profile_last_name,
            profile_avatar_url,
            profile_bio
        }
    }
}
