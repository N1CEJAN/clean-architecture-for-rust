use std::sync::Arc;

use log::debug;
use uuid::Uuid;

use crate::business::error::BusinessError;
use crate::business::user::repository::UserRepository;
use crate::business::user::request::{CreateUserRequest, DeleteUserRequest};
use crate::core::user::{User, UserDto};

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
    pub async fn index(&self) -> Result<Vec<UserDto>, BusinessError> {
        debug!("UserService.index()");
        let vec_of_user = self.user_repository.find_all().await?;
        Ok(vec_of_user.iter().map(|user| user.to_dto()).collect())
    }
    pub async fn show(&self, id: Uuid) -> Result<Option<UserDto>, BusinessError> {
        Ok(self
            .user_repository
            .find_by_id(&id)
            .await?
            .map(|user| user.to_dto()))
    }
    pub async fn create(&self, request: CreateUserRequest) -> Result<UserDto, BusinessError> {
        let password = request.password().to_string();
        let new_user = User::new(request.username(), password.as_str());
        self.user_repository.create(&new_user).await?;
        Ok(new_user.to_dto())
    }
    pub async fn delete(&self, request: DeleteUserRequest) -> Result<(), BusinessError> {
        self.user_repository
            .delete_by_id(request.user_id())
            .await
            .map_err(|error| BusinessError::from(error))
    }
}
