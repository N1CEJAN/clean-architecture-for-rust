use std::future::{ready, Ready};
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{FromRequest, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::core::error::AuthenticationError;

const JWT_ISSUER: &str = "asdf";
const JWT_TTL_IN_MILLIS: u128 = 1000 * 60 * 15;
const JWT_SECRET: &str = "secret";

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
        self.claims.sub.as_str()
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
            key: key.to_owned(),
            claims,
        })
    }
}

impl FromRequest for JsonWebToken {
    type Error = AuthenticationError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(header_value) = header.to_str() {
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
            iss: JWT_ISSUER.to_owned(),
            iat: now,
            exp: now + JWT_TTL_IN_MILLIS,
            sub: sub.to_owned(),
        }
    }
}
