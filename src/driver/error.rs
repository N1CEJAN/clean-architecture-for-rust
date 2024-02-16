use deadpool_postgres::PoolError;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct DriverError {
    message: String,
}

impl DriverError {
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<PoolError> for DriverError {
    fn from(error: PoolError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<Error> for DriverError {
    fn from(error: Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
