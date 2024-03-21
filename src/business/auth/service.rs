use std::sync::Arc;

use crate::business::auth::request::{LoginUserRequest, RegisterUserRequest};
use crate::business::error::BusinessError;
use crate::business::user::repository::UserRepository;
use crate::core::error::AuthenticationError;
use crate::core::user::{User, UserDto};

pub struct AuthService {
    user_repository: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
    pub async fn register(&self, request: RegisterUserRequest) -> Result<UserDto, BusinessError> {
        let new_user = User::new(request.username(), request.password());
        self.user_repository.create(&new_user).await?;
        Ok(new_user.to_dto())
    }
    pub async fn login(&self, request: LoginUserRequest) -> Result<UserDto, BusinessError> {
        println!("AuthService -> login");
        let mut user = self
            .user_repository
            .find_by_username(request.username())
            .await?
            .ok_or(AuthenticationError::new("invalid credentials"))?;
        user.login(request.password())?;
        self.user_repository.update(&user).await?;
        Ok(user.to_dto())
    }
    pub async fn refresh(&self, old_refresh_token: &str) -> Result<UserDto, BusinessError> {
        let mut user = self
            .user_repository
            .find_by_token(old_refresh_token)
            .await?
            .ok_or(AuthenticationError::new("invalid token"))?;
        user.refresh(old_refresh_token)?;
        self.user_repository.update(&user).await?;
        Ok(user.to_dto())
    }
    pub async fn logout(&self, refresh_token: &str) -> Result<(), BusinessError> {
        let mut user = self
            .user_repository
            .find_by_token(refresh_token)
            .await?
            .ok_or(AuthenticationError::new("invalid token"))?;
        user.logout(refresh_token)?;
        self.user_repository.update(&user).await?;
        Ok(())
    }
}
