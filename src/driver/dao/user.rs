use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use crate::business::dao::user::UserDao;
use crate::core::user::UserDto;
use crate::driver::database::client_adapter::ClientAdapter;
use crate::driver::database::pool_adapter::PoolAdapter;
use crate::driver::error::DriverError;

pub struct RawUserDao {
    pool: Arc<PoolAdapter>,
}

impl RawUserDao {
    pub fn new(pool: Arc<PoolAdapter>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserDao for RawUserDao {
    async fn create(&self, user_dto: &UserDto) -> Result<(), DriverError> {
        let statement = "INSERT INTO Users VALUES ($1, $2, $3)";
        let values: [&(dyn ToSql + Sync); 3] =
            [&user_dto.id(), &user_dto.username(), &user_dto.password()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<UserDto>, DriverError> {
        let statement = "SELECT * FROM Users WHERE id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&id]).await?;
        Ok(rows.first().map(|row| UserDto::from(row)))
    }

    async fn find_all(&self) -> Result<Vec<UserDto>, DriverError> {
        let statement = "SELECT * FROM Users";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[]).await?;
        Ok(rows.iter().map(|row| UserDto::from(row)).collect())
    }

    async fn update(&self, user_dto: &UserDto) -> Result<(), DriverError> {
        let statement = "UPDATE Users SET username=$2, password=$3 WHERE id=$1";
        let values: [&(dyn ToSql + Sync); 3] =
            [&user_dto.id(), &user_dto.username(), &user_dto.password()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), DriverError> {
        let statement = "DELETE FROM Users WHERE id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &[&id]).await?;
        Ok(())
    }
}
