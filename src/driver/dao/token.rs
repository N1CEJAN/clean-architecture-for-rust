use std::sync::Arc;

use log::debug;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use crate::core::token::TokenDto;
use crate::driver::database::client_adapter::ClientAdapter;
use crate::driver::database::pool_adapter::PoolAdapter;
use crate::driver::error::DriverError;

#[derive(Debug)]
pub struct TokenDao {
    pool: Arc<PoolAdapter>,
}

impl TokenDao {
    pub fn new(pool: Arc<PoolAdapter>) -> Self {
        Self { pool }
    }
    pub async fn create(&self, token_dto: &TokenDto) -> Result<(), DriverError> {
        debug!("TokenDao.create() with inputs: token_dto={:?}", token_dto);
        let statement = "INSERT INTO Tokens VALUES ($1, $2, $3, $4, $5)";
        let values: [&(dyn ToSql + Sync); 5] = [
            &token_dto.id(),
            &token_dto.key(),
            &token_dto.user_id(),
            &token_dto.expire_at(),
            &token_dto.is_revoked(),
        ];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn save(&self, token_dto: &TokenDto) -> Result<(), DriverError> {
        debug!("TokenDao.save() with inputs: token_dto={:?}", token_dto);
        let statement = r#"
            INSERT INTO Tokens VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET key = EXCLUDED.key,
                user_id = EXCLUDED.user_id,
                expire_at = EXCLUDED.expire_at,
                is_revoked = EXCLUDED.is_revoked
        "#;
        let values: [&(dyn ToSql + Sync); 5] = [
            &token_dto.id(),
            &token_dto.key(),
            &token_dto.user_id(),
            &token_dto.expire_at(),
            &token_dto.is_revoked(),
        ];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), DriverError> {
        debug!("TokenDao.delete_by_user_id() with inputs: user_id={user_id:?}");
        let statement = "DELETE FROM Tokens WHERE user_id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &[&user_id]).await?;
        Ok(())
    }
    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TokenDto>, DriverError> {
        debug!("TokenDao.find_by_user_id() with inputs: user_id={user_id:?}");
        let statement = "SELECT * FROM Tokens WHERE user_id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&user_id]).await?;
        let result = Ok(rows.into_iter().map(|row| TokenDto::from(&row)).collect());
        debug!("TokenDao.find_by_user_id() with output: {:?}", result);
        result
    }
}
