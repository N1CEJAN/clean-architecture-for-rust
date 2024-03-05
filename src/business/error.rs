use serde::Serialize;

use crate::core::error::AuthenticationError;
use crate::driver::error::DriverError;

#[derive(Debug, Serialize)]
pub struct BusinessError {
    message: String,
}

impl std::fmt::Display for BusinessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl From<DriverError> for BusinessError {
    fn from(value: DriverError) -> Self {
        Self {
            message: value.message().to_string(),
        }
    }
}

impl From<AuthenticationError> for BusinessError {
    fn from(value: AuthenticationError) -> Self {
        Self {
            message: value.message().to_string().clone(),
        }
    }
}
