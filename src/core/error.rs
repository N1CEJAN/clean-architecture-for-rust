use std::fmt::{Debug, Display, Formatter};

use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Debug)]
pub struct AuthenticationError {
    message: String,
}

impl AuthenticationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string().clone(),
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
