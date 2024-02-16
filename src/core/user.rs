use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            username,
            password,
        }
    }
}

impl From<&UserDto> for User {
    fn from(dto: &UserDto) -> Self {
        Self {
            id: dto.id().clone(),
            username: dto.username().to_string(),
            password: dto.password().to_string(),
        }
    }
}

impl Into<UserDto> for User {
    fn into(self) -> UserDto {
        UserDto::new(
            self.id,
            self.username,
            self.password,
        )
    }
}

#[derive(Serialize)]
pub struct UserDto {
    id: Uuid,
    username: String,
    password: String,
}

impl UserDto {
    fn new(id: Uuid, username: String, password: String) -> Self {
        Self {
            id,
            username,
            password,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

impl From<&Row> for UserDto {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get(0),
            username: value.get(1),
            password: value.get(2),
        }
    }
}
