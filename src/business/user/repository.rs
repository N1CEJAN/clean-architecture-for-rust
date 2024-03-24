use log::debug;
use uuid::Uuid;

use crate::core::user::User;
use crate::driver::dao::token::TokenDao;
use crate::driver::dao::user::UserDao;
use crate::driver::error::DriverError;

#[derive(Debug)]
pub struct UserRepository {
    user_dao: UserDao,
    token_dao: TokenDao,
}

impl UserRepository {
    pub fn new(user_dao: UserDao, token_dao: TokenDao) -> Self {
        Self {
            user_dao,
            token_dao,
        }
    }
    pub async fn create(&self, user: &User) -> Result<(), DriverError> {
        debug!("UserRepository.create() with inputs: user={:?}", user);
        let user_dto = user.to_dto();
        for token_dto in user_dto.tokens() {
            self.token_dao.create(token_dto).await?;
        }
        self.user_dao.create(&user_dto).await
    }
    pub async fn update(&self, user: &User) -> Result<(), DriverError> {
        debug!("UserRepository.update() with inputs: user={:?}", user);
        let user_dto = user.to_dto();
        for token_dto in user_dto.tokens() {
            self.token_dao.update(token_dto).await?;
        }
        self.user_dao.update(&user_dto).await
    }
    pub async fn delete_by_id(&self, user_id: &Uuid) -> Result<(), DriverError> {
        debug!("UserRepository.delete_by_id() with inputs: user_id={:?}", user_id);
        self.user_dao.delete_by_id(user_id).await?;
        self.token_dao.delete_by_user_id(user_id).await?;
        Ok(())
    }
    pub async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, DriverError> {
        debug!("UserRepository.find_by_id() with inputs: user_id={:?}", user_id);
        let user_dto = self.user_dao.find_by_id(user_id).await?;
        let vec_of_token_dtos = self.token_dao.find_by_user_id(user_id).await?;
        Ok(user_dto.map(move |user_dto| User::from_dto(&user_dto, &vec_of_token_dtos)))
    }
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, DriverError> {
        debug!("UserRepository.find_by_username() with inputs: username={:?}", username);
        if let Some(user_dto) = self.user_dao.find_by_username(username).await? {
            let vec_of_token_dtos = self.token_dao.find_by_user_id(user_dto.id()).await?;
            return Ok(Some(User::from_dto(&user_dto, &vec_of_token_dtos)));
        }
        Ok(None)
    }
    pub async fn find_by_token(&self, key: &str) -> Result<Option<User>, DriverError> {
        debug!("UserRepository.find_by_username() with inputs: key={:?}", key);
        if let Some(user_dto) = self.user_dao.find_by_token(key).await? {
            let vec_of_token_dtos = self.token_dao.find_by_user_id(user_dto.id()).await?;
            return Ok(Some(User::from_dto(&user_dto, &vec_of_token_dtos)));
        }
        Ok(None)
    }
    pub async fn find_all(&self) -> Result<Vec<User>, DriverError> {
        debug!("UserRepository.find_all()");
        let user_dtos = self.user_dao.find_all().await?;
        let mut vec_of_user = vec!();
        for user_dto in user_dtos {
            let vec_of_token_dtos = self.token_dao.find_by_user_id(user_dto.id()).await?;
            vec_of_user.push(User::from_dto(&user_dto, &vec_of_token_dtos))
        }
        let result = Ok(vec_of_user);
        debug!("UserRepository.find_all() with output: {:?}", result);
        result
    }
}
