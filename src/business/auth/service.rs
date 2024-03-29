use std::sync::Arc;

use log::debug;

use crate::business::auth::request::LoginUserRequest;
use crate::business::error::BusinessError;
use crate::business::user::repository::UserRepository;
use crate::core::error::AuthenticationError;
use crate::core::user::UserDto;

#[derive(Debug)]
pub struct AuthService {
    user_repository: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
    pub async fn login(&self, request: LoginUserRequest) -> Result<UserDto, BusinessError> {
        debug!("AuthService.login() with inputs: request={:?}", request);
        let mut user = self.user_repository.find_by_username(request.username()).await?
            .ok_or(AuthenticationError::new("invalid credentials"))?;
        user.login(request.password())?;
        self.user_repository.update(&user).await?;
        Ok(user.to_dto())
    }
    pub async fn refresh(&self, refresh_token: &str) -> Result<UserDto, BusinessError> {
        debug!("AuthService.refresh() with inputs: refresh_token={:?}", refresh_token);
        let mut user = self.user_repository.find_by_token(refresh_token).await?
            .ok_or(AuthenticationError::new("invalid token"))?;
        user.refresh(refresh_token)?;
        self.user_repository.update(&user).await?;
        Ok(user.to_dto())
    }
    pub async fn logout(&self, refresh_token: &str) -> Result<(), BusinessError> {
        debug!("AuthService.logout() with inputs: refresh_token={:?}", refresh_token);
        let mut user = self.user_repository.find_by_token(refresh_token).await?
            .ok_or(AuthenticationError::new("invalid token"))?;
        user.logout(refresh_token)?;
        self.user_repository.update(&user).await?;
        Ok(())
    }
}
