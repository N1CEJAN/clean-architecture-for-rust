use deadpool_postgres::Object as Client;
use tokio_postgres::{Row, Statement};
use tokio_postgres::types::ToSql;

use crate::driver::error::DriverError;

pub struct ClientAdapter;

impl ClientAdapter {
    pub async fn prepare(client: &mut Client, statement: &str) -> Result<Statement, DriverError> {
        client
            .prepare(statement)
            .await
            .map_err(DriverError::from)
    }

    pub async fn execute(
        client: &mut Client,
        stmt: Statement,
        values: &[&(dyn ToSql + Sync)],
    ) -> Result<(), DriverError> {
        client
            .execute(&stmt, values)
            .await
            .map_err(DriverError::from)?;
        Ok(())
    }

    pub async fn query(
        client: &mut Client,
        stmt: Statement,
        values: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, DriverError> {
        client
            .query(&stmt, values)
            .await
            .map_err(DriverError::from)
    }
}
