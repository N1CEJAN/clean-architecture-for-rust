use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct RegisterUserRequest {
    username: String,
    password: String,
}

impl RegisterUserRequest {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteUserRequest {
    user_id: Uuid,
}

impl DeleteUserRequest {
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
}
