use deadpool_postgres::{Object, Pool};

use crate::driver::error::DriverError;

#[derive(Debug)]
pub struct PoolAdapter {
    pool: Pool,
}

impl PoolAdapter {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_connection(&self) -> Result<Object, DriverError> {
        self.pool
            .get()
            .await
            .map_err(DriverError::from)
    }
}
