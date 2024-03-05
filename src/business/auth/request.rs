use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    username: String,
    password: String,
}

impl RegisterUserRequest {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string().clone(),
            password: password.to_string().clone(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    username: String,
    password: String,
}

impl LoginUserRequest {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string().clone(),
            password: password.to_string().clone(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

