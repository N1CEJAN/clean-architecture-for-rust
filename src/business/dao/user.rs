use async_trait::async_trait;
use uuid::Uuid;

use crate::core::user::UserDto;
use crate::driver::error::DriverError;

#[async_trait]
pub trait UserDao: Send + Sync {
    async fn create(&self, user_dto: &UserDto) -> Result<(), DriverError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<UserDto>, DriverError>;
    async fn find_all(&self) -> Result<Vec<UserDto>, DriverError>;
    async fn update(&self, user_dto: &UserDto) -> Result<(), DriverError>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<(), DriverError>;
}
