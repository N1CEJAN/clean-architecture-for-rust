use std::sync::Arc;

use deadpool_postgres::{Object, Pool};

use crate::driver::error::DriverError;

pub struct PoolAdapter {
    pool: Arc<Pool>,
}

impl PoolAdapter {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_connection(&self) -> Result<Object, DriverError> {
        self.pool
            .get()
            .await
            .map_err(|error| DriverError::from(error))
    }
}
