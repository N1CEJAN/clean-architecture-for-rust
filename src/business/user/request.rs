use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

impl CreateUserRequest {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    user_id: Uuid,
}

impl DeleteUserRequest {
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
}


