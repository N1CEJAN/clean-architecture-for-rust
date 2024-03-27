use bcrypt::{hash, verify};
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::core::error::AuthenticationError;

use crate::core::token::{Token, TokenDto};

#[derive(Debug)]
pub struct User {
    id: Uuid,
    username: String,
    password: String,
    tokens: Vec<Token>
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            username: username.to_string().clone(),
            password: hash(password.to_string().clone(), 12).unwrap(),
            tokens: Vec::with_capacity(1)
        }
    }
    pub fn from_dto(user_dto: &UserDto, list_of_token_dto: &Vec<TokenDto>) -> Self {
        Self {
            id: user_dto.id().clone(),
            username: user_dto.username().to_string().clone(),
            password: user_dto.password().to_string().clone(),
            tokens: list_of_token_dto.iter()
                .map(|dto| Token::from_dto(dto))
                .collect(),
        }
    }
    pub fn to_dto(&self) -> UserDto {
        UserDto::new(
            &self.id,
            self.username.as_str(),
            self.password.as_str(),
            &self.tokens.iter()
                .map(|token| token.to_dto())
                .collect(),
        )
    }
    pub fn login(&mut self, password: &str) -> Result<(), AuthenticationError> {
        if verify(password, self.password.as_str()).unwrap() {
            let refresh_token = Token::new(&self.id);
            self.tokens.push(refresh_token);
            return Ok(())
        }
        Err(AuthenticationError::new("invalid credentials"))
    }
    pub fn refresh(&mut self, old_token_key: &str) -> Result<(), AuthenticationError> {
        if let Some(old_token) = self.token_by_key(old_token_key) {
            old_token.validate()?;
            old_token.revoke();
            let new_token = Token::new(&self.id);
            self.tokens.push(new_token);
            return Ok(())
        }
        Err(AuthenticationError::new("invalid token"))
    }
    pub fn logout(&mut self, token_key: &str) -> Result<(), AuthenticationError> {
        if let Some(old_token) = self.token_by_key(token_key) {
            old_token.revoke();
            return Ok(())
        }
        Err(AuthenticationError::new("invalid token"))
    }
    fn token_by_key(&mut self, key: &str) -> Option<&mut Token> {
        self.tokens.iter_mut().find(|token| token.matches(key))
    }
}

#[derive(Serialize, Debug)]
pub struct UserDto {
    id: Uuid,
    username: String,
    password: String,
    tokens: Vec<TokenDto>,
}

impl UserDto {
    fn new(id: &Uuid, username: &str, password: &str, tokens: &Vec<TokenDto>) -> Self {
        Self {
            id: id.clone(),
            username: username.to_string().clone(),
            password: password.to_string().clone(),
            tokens: tokens.to_vec(),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn tokens(&self) -> &Vec<TokenDto> {
        &self.tokens
    }
    pub fn latest_token(&self) -> Option<&TokenDto> {
        self.tokens.last()
    }
}

impl From<&Row> for UserDto {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get(0),
            username: value.get(1),
            password: value.get(2),
            tokens: Vec::with_capacity(1),
        }
    }
}
