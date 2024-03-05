use std::sync::Arc;

use tokio_postgres::types::ToSql;
use uuid::Uuid;

use crate::core::token::TokenDto;
use crate::driver::database::client_adapter::ClientAdapter;
use crate::driver::database::pool_adapter::PoolAdapter;
use crate::driver::error::DriverError;

pub struct TokenDao {
    pool: Arc<PoolAdapter>,
}

impl TokenDao {
    pub fn new(pool: Arc<PoolAdapter>) -> Self {
        Self { pool }
    }
    pub async fn create(&self, token_dto: &TokenDto) -> Result<(), DriverError> {
        let statement = "INSERT INTO Token VALUES ($1, $2, $3)";
        let values: [&(dyn ToSql + Sync); 3] =
            [&token_dto.id(), &token_dto.key(), &token_dto.user_id()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn update(&self, token_dto: &TokenDto) -> Result<(), DriverError> {
        let statement = "UPDATE Tokens SET key=$2, user_id=$3 WHERE id=$1";
        let values: [&(dyn ToSql + Sync); 3] =
            [&token_dto.id(), &token_dto.key(), &token_dto.user_id()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TokenDto>, DriverError> {
        let statement = "DELETE FROM Token WHERE user_id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&user_id]).await?;
        Ok(rows.into_iter().map(|row| TokenDto::from(&row)).collect())
    }
    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TokenDto>, DriverError> {
        let statement = "SELECT * FROM Token WHERE user_id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&user_id]).await?;
        Ok(rows.into_iter().map(|row| TokenDto::from(&row)).collect())
    }
}
