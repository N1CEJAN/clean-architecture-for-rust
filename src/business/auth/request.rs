use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginUserRequest {
    username: String,
    password: String,
}

impl LoginUserRequest {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
