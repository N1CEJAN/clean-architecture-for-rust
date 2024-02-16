use std::sync::Arc;

use uuid::Uuid;

use crate::api::request::user::CreateUserRequest;
use crate::api::request::user::DeleteUserRequest;
use crate::business::dao::user::UserDao;
use crate::business::error::BusinessError;
use crate::core::user::{User, UserDto};

pub struct UserService {
    user_dao: Arc<dyn UserDao>,
}

impl UserService {
    pub fn new(user_dao: Arc<dyn UserDao>) -> Self {
        Self { user_dao }
    }

    pub async fn index(&self) -> Result<Vec<UserDto>, BusinessError> {
        self.user_dao
            .find_all()
            .await
            .map_err(|error| BusinessError::from(error))
    }

    pub async fn show(&self, id: Uuid) -> Result<Option<UserDto>, BusinessError> {
        self.user_dao
            .find_by_id(&id)
            .await
            .map_err(|error| BusinessError::from(error))
    }

    pub async fn create(&self, request: CreateUserRequest) -> Result<UserDto, BusinessError> {
        let username = request.username().to_string();
        let password = request.password().to_string();
        let new_user = User::new(username, password);
        let user_dto = new_user.into();
        self.user_dao
            .create(&user_dto)
            .await
            .map_err(|error| BusinessError::from(error))?;
        Ok(user_dto)
    }

    pub async fn delete(&self, request: DeleteUserRequest) -> Result<(), BusinessError> {
        self.user_dao
            .delete_by_id(request.user_id())
            .await
            .map_err(|error| BusinessError::from(error))
    }
}
