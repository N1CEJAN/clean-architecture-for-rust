use std::borrow::Cow;
use std::future::{ready, Ready};
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{FromRequest, HttpRequest};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::dev::Payload;
use actix_web::http::header;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::core::error::AuthenticationError;
use crate::core::token::TokenDto;

const JWT_ISSUER: &str = "asdf";
const JWT_TTL_IN_MILLIS: u128 = 1000 * 60 * 15;
const JWT_SECRET: &str = "secret";

const RT_COOKIE_NAME: &str = "refresh-token";
const RT_COOKIE_HTTP_ONLY: bool = true;

#[derive(Debug)]
pub struct RefreshToken<'a> {
    cookie: Cookie<'a>,
}

impl<'a> RefreshToken<'a> {
    pub fn new(token_dto: &'a TokenDto) -> Self {
        let key = urlencoding::encode(token_dto.key());
        let ttl = token_dto.expire_at().duration_since(SystemTime::now())
            .expect("cannot be created from expired token");

        let mut cookie = Cookie::new(RT_COOKIE_NAME, key);
        cookie.set_http_only(RT_COOKIE_HTTP_ONLY);
        cookie.set_max_age(Duration::try_from(ttl)
            .expect("this obscure error should simply not happen"));
        Self { cookie }
    }
    pub fn cookie(&self) -> &Cookie {
        &self.cookie
    }
    pub fn key(&self) -> Cow<str> {
        urlencoding::decode(self.cookie.value()).unwrap()
    }
}

impl<'a> FromRequest for RefreshToken<'a> {
    type Error = AuthenticationError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(cookie) = request.cookie("refresh-token") {
            return ready(Ok(Self { cookie }));
        }
        ready(Err(AuthenticationError::new(
            "could not read refresh token",
        )))
    }
}

pub struct JsonWebToken {
    key: String,
    claims: Claims,
}

impl JsonWebToken {
    pub fn new(username: &str) -> Self {
        let claims = Claims::new(username);
        JsonWebToken::encode(claims)
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn username(&self) -> &str {
        &self.claims.sub.as_str()
    }
    fn encode(claims: Claims) -> Self {
        let key = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )
        .unwrap();
        Self { key, claims }
    }
    fn decode(key: &str) -> Result<Self, AuthenticationError> {
        let claims = decode::<Claims>(
            key,
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| AuthenticationError::new(err.to_string().as_str()))?
        .claims;

        Ok(Self {
            key: key.to_string().clone(),
            claims,
        })
    }
}

impl FromRequest for JsonWebToken {
    type Error = AuthenticationError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header) = req.headers().get(header::AUTHORIZATION) {
            if let Some(header_value) = header.to_str().ok() {
                debug!("{:?}", &header_value[7..]);
                return ready(JsonWebToken::decode(&header_value[7..]));
            }
        }
        ready(Err(AuthenticationError::new(
            "could not read json web token",
        )))
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: u128,
    exp: u128,
    sub: String,
}

impl Claims {
    fn new(sub: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            iss: JWT_ISSUER.to_string(),
            iat: now,
            exp: now + JWT_TTL_IN_MILLIS,
            sub: sub.to_string().clone(),
        }
    }
}
