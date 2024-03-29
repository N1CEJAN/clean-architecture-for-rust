use std::sync::Arc;

use log::debug;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use crate::core::user::UserDto;
use crate::driver::database::client_adapter::ClientAdapter;
use crate::driver::database::pool_adapter::PoolAdapter;
use crate::driver::error::DriverError;

#[derive(Debug)]
pub struct UserDao {
    pool: Arc<PoolAdapter>,
}

impl UserDao {
    pub fn new(pool: Arc<PoolAdapter>) -> Self {
        Self { pool }
    }
    pub async fn create(&self, user_dto: &UserDto) -> Result<(), DriverError> {
        debug!("UserDao.create() with inputs: user_dto={:?}", user_dto);
        let statement = "INSERT INTO Users VALUES ($1, $2, $3)";
        let values: [&(dyn ToSql + Sync); 3] =
            [&user_dto.id(), &user_dto.username(), &user_dto.password()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<UserDto>, DriverError> {
        debug!("UserDao.find_by_id() with inputs: id={:?}", id);
        let statement = "SELECT * FROM Users WHERE id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[id]).await?;
        let result = Ok(rows.first().map(UserDto::from));
        debug!("UserDao.find_by_id() with output: {:?}", result);
        result
    }
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserDto>, DriverError> {
        debug!(
            "UserDao.find_by_username() with inputs: username={:?}",
            username
        );
        let statement = "SELECT * FROM Users WHERE username=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&username]).await?;
        let result = Ok(rows.first().map(UserDto::from));
        debug!("UserDao.find_by_username() with output: {:?}", result);
        result
    }
    pub async fn find_by_token(&self, key: &str) -> Result<Option<UserDto>, DriverError> {
        debug!("UserDao.find_by_token() with inputs: key={:?}", key);
        let statement =
            "SELECT u.* FROM Users u INNER JOIN Tokens t ON u.id = t.user_id WHERE t.key=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[&key]).await?;
        let result = Ok(rows.first().map(UserDto::from));
        debug!("UserDao.find_by_token() with output: {:?}", result);
        result
    }
    pub async fn find_all(&self) -> Result<Vec<UserDto>, DriverError> {
        debug!("UserDao.find_all() with no inputs");
        let statement = "SELECT * FROM Users";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        let rows = ClientAdapter::query(&mut client, stmt, &[]).await?;
        let result = Ok(rows.iter().map(UserDto::from).collect());
        debug!("UserDao.find_all() with output: {:?}", result);
        result
    }
    pub async fn update(&self, user_dto: &UserDto) -> Result<(), DriverError> {
        debug!("UserDao.update() with inputs: user_dto={:?}", user_dto);
        let statement = "UPDATE Users SET username=$2, password=$3 WHERE id=$1";
        let values: [&(dyn ToSql + Sync); 3] =
            [&user_dto.id(), &user_dto.username(), &user_dto.password()];
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &values).await?;
        Ok(())
    }
    pub async fn delete_by_id(&self, id: &Uuid) -> Result<(), DriverError> {
        debug!("UserDao.delete_by_id() with inputs: id={:?}", id);
        let statement = "DELETE FROM Users WHERE id=$1";
        let mut client = self.pool.get_connection().await?;
        let stmt = ClientAdapter::prepare(&mut client, statement).await?;
        ClientAdapter::execute(&mut client, stmt, &[&id]).await?;
        Ok(())
    }
}
