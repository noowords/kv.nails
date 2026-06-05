use uuid::{ Uuid };

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
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

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl std::str::FromStr for UserId {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(UserId)
            .map_err(|e| e.to_string())
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}
