use rand::random;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct Token {
    id: Uuid,
    key: String,
    user_id: Uuid,
    is_revoked: bool,
}

impl Token {
    pub fn new(user_id: &Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            key: String::from_utf8(random::<[u8; 32]>().to_vec()).unwrap(),
            user_id: user_id.clone(),
            is_revoked: false,
        }
    }
    pub fn from_dto(token_dto: &TokenDto) -> Self {
        Self {
            id: token_dto.id().clone(),
            key: token_dto.key().to_string().clone(),
            user_id: token_dto.user_id().clone(),
            is_revoked: token_dto.is_revoked().clone(),
        }
    }
    pub fn to_dto(&self) -> TokenDto {
        TokenDto::new(&self.id, self.key.as_str(), &self.user_id, &self.is_revoked)
    }
    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
    pub fn matches(&self, key: &str) -> bool {
        &self.key == key
    }
}

#[derive(Serialize, Clone)]
pub struct TokenDto {
    id: Uuid,
    key: String,
    user_id: Uuid,
    is_revoked: bool,
}

impl TokenDto {
    fn new(id: &Uuid, key: &str, user_id: &Uuid, is_revoked: &bool) -> Self {
        Self {
            id: id.clone(),
            key: key.to_string().clone(),
            user_id: user_id.clone(),
            is_revoked: is_revoked.clone(),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
    pub fn is_revoked(&self) -> bool {
        self.is_revoked
    }
}

impl From<&Row> for TokenDto {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get(0),
            key: value.get(1),
            user_id: value.get(2),
            is_revoked: value.get(3),
        }
    }
}
