use crate::core::error::AuthenticationError;
use crate::core::token::TokenDto;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::borrow::Cow;
use std::future::{ready, Ready};
use std::time::SystemTime;

const RT_COOKIE_NAME: &str = "refresh-token";
const RT_COOKIE_HTTP_ONLY: bool = true;

#[derive(Debug)]
pub struct RefreshToken<'a> {
    cookie: Cookie<'a>,
}

impl<'a> RefreshToken<'a> {
    pub fn new(token_dto: &'a TokenDto) -> Self {
        let key = urlencoding::encode(token_dto.key());
        let ttl = token_dto
            .expire_at()
            .duration_since(SystemTime::now())
            .expect("System clock may have gone backwards");
        let max_age = Duration::try_from(ttl).expect("ttl is expected to be valid");

        let mut cookie = Cookie::new(RT_COOKIE_NAME, key);
        cookie.set_http_only(RT_COOKIE_HTTP_ONLY);
        cookie.set_max_age(max_age);
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
