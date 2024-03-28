use std::time::{Duration, SystemTime};

use rand::{Rng, thread_rng};
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::core::error::AuthenticationError;

const TOKEN_TTL: Duration = Duration::from_secs(60 * 60); // h = m * s

#[derive(Debug)]
pub struct Token {
    id: Uuid,
    key: String,
    user_id: Uuid,
    expire_at: SystemTime,
    is_revoked: bool,
}

impl Token {
    pub fn new(user_id: &Uuid) -> Self {
        let mut rng = thread_rng();
        let key = (0..32)
            .map(|_| rng.gen_range(0x0020..0x007E)) // UTF-8 characters in printable ASCII range
            .map(|c| char::from_u32(c).unwrap())
            .collect();

        Self {
            id: Uuid::now_v7(),
            key,
            user_id: user_id.clone(),
            expire_at: SystemTime::now() + TOKEN_TTL,
            is_revoked: false,
        }
    }
    pub fn from_dto(token_dto: &TokenDto) -> Self {
        Self {
            id: token_dto.id().clone(),
            key: token_dto.key().to_string().clone(),
            user_id: token_dto.user_id().clone(),
            expire_at: token_dto.expire_at().clone(),
            is_revoked: token_dto.is_revoked().clone(),
        }
    }
    pub fn to_dto(&self) -> TokenDto {
        TokenDto::new(&self.id, self.key.as_str(), &self.user_id, &self.expire_at, &self.is_revoked)
    }
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        if self.is_revoked || SystemTime::now() > self.expire_at {
            return Err(AuthenticationError::new("invalid token"))
        }
        Ok(())
    }
    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
    pub fn matches(&self, key: &str) -> bool {
        self.key == key
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenDto {
    id: Uuid,
    key: String,
    user_id: Uuid,
    expire_at: SystemTime,
    is_revoked: bool,
}

impl TokenDto {
    fn new(id: &Uuid, key: &str, user_id: &Uuid, expire_at: &SystemTime, is_revoked: &bool) -> Self {
        Self {
            id: id.clone(),
            key: key.to_string().clone(),
            user_id: user_id.clone(),
            expire_at: expire_at.clone(),
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
    pub fn expire_at(&self) -> &SystemTime {
        &self.expire_at
    }
    pub fn is_revoked(&self) -> &bool {
        &self.is_revoked
    }
}

impl From<&Row> for TokenDto {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get(0),
            key: value.get(1),
            user_id: value.get(2),
            expire_at: value.get(3),
            is_revoked: value.get(4),
        }
    }
}
